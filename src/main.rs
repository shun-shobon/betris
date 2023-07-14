pub mod block;
pub mod field;
pub mod mino;
pub mod position;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::camera::ScalingMode,
};

#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, fps_system)
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1000.);
    commands.spawn(camera_bundle);

    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font_size: 20.,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: 20.,
                    color: Color::WHITE,
                    ..default()
                }),
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

fn fps_system(diagnostic: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    let Some(fps) = diagnostic.get(FrameTimeDiagnosticsPlugin::FPS) else { return; };
    let Ok(mut fps_text) = query.get_single_mut() else { return; };

    fps_text.sections[1].value = format!("{:.2}", fps.average().unwrap_or_default());
}
