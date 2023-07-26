use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

const FPS_TEXT_COLOR: Color = Color::GREEN;

#[derive(Component)]
pub struct FpsText;

pub fn setup_fps(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: 20.,
        color: FPS_TEXT_COLOR,
        ..default()
    };

    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new("FPS: ", text_style.clone()),
                TextSection::from_style(text_style),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.),
                left: Val::Px(5.),
                ..default()
            }),
        )
        .insert(FpsText);
}

pub fn fps_system(diagnostic: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    let Some(fps) = diagnostic.get(FrameTimeDiagnosticsPlugin::FPS) else { return; };
    let Ok(mut fps_text) = query.get_single_mut() else { return; };

    fps_text.sections[1].value = format!("{:.2}", fps.average().unwrap_or_default());
}
