use crate::{
    field::{local::LocalField, Field},
    net::{broadcast_state, PlayerId, PlayerState, Players, Socket},
};
use bevy::prelude::*;
use if_chain::if_chain;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MatchMaking,
    Playing,
    Finished,
}

#[derive(Event)]
pub struct GameOverEvent;

#[derive(Event)]
pub struct StateChangeEvent {
    pub player_id: PlayerId,
    pub state: PlayerState,
}

pub fn handle_gameover(
    mut events: EventReader<GameOverEvent>,
    mut state: ResMut<NextState<AppState>>,
    mut socket: ResMut<Socket>,
    players: Res<Players>,
    mut field_query: Query<&mut Field, With<LocalField>>,
) {
    if events.iter().next().is_none() {
        return;
    }
    let Ok(mut field) = field_query.get_single_mut() else { return; };

    field.player.state = PlayerState::GameOver;
    broadcast_state(&mut socket, &players, PlayerState::GameOver);

    state.set(AppState::Finished);
}

pub fn handle_state_change(
    mut events: EventReader<StateChangeEvent>,
    mut state: ResMut<NextState<AppState>>,
    mut socket: ResMut<Socket>,
    mut players: ResMut<Players>,
    mut field_query: Query<&mut Field, Without<LocalField>>,
    mut my_field_query: Query<&mut Field, With<LocalField>>,
) {
    for event in events.iter() {
        if_chain! {
            if let Some(mut field) = field_query.iter_mut().find(|field| field.player.id == event.player_id);
            if let Some(player) = players.0.iter_mut().find(|player| player.id == event.player_id);
            then {
                field.player.state = event.state;
                player.state = event.state;
            }
        }

        if players
            .0
            .iter()
            .all(|player| player.state == PlayerState::GameOver)
        {
            let Ok(mut my_field) = my_field_query.get_single_mut() else { return; };

            my_field.player.state = PlayerState::Win;
            broadcast_state(&mut socket, &players, PlayerState::Win);

            state.set(AppState::Finished);
        }
    }
}
