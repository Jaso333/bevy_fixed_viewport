use bevy::{
    color::palettes::tailwind,
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use bevy_fixed_viewport::{FixedViewport, FixedViewportPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FixedViewportPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, change_aspect_ratio)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        FixedViewport {
            aspect_ratio: 16. / 9.,
        },
    ));

    // create a rectangle that will always fill the screen
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::Srgba(tailwind::RED_500),
            custom_size: Some(Vec2::new(10000., 10000.)),
            ..default()
        },
        ..default()
    });
}

fn change_aspect_ratio(
    mut input_events: EventReader<KeyboardInput>,
    mut camera_query: Query<&mut FixedViewport>,
) {
    for event in input_events.read() {
        match match event.state {
            ButtonState::Pressed => match event.key_code {
                KeyCode::ArrowUp => Some(0.1),
                KeyCode::ArrowDown => Some(-0.1),
                _ => None,
            },
            _ => continue,
        } {
            Some(change) => {
                for mut fixed_viewport in camera_query.iter_mut() {
                    fixed_viewport.aspect_ratio += change;
                }
            }
            None => continue,
        }
    }
}
