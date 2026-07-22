use bevy::{camera::ScalingMode, prelude::*};
use bevy_fixed_viewport::{FixedViewport, FixedViewportPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FixedViewportPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, change_ratio)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.5, 0.0, 0.75)),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 1920.0,
                height: 1080.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        FixedViewport {
            aspect_ratio: 1920.0 / 1080.0,
        },
    ));
}

fn change_ratio(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut FixedViewport>) {
    let mut direction = 0f32;

    if input.just_pressed(KeyCode::ArrowUp) {
        direction += 1.0;
    }
    if input.just_pressed(KeyCode::ArrowDown) {
        direction -= 1.0;
    }

    if direction == 0.0 {
        return;
    }

    for mut fixed_viewport in query.iter_mut() {
        fixed_viewport.aspect_ratio += direction * 0.1;
    }
}
