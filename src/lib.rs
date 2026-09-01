use bevy::{
    camera::{RenderTarget, Viewport},
    prelude::*,
    window::PrimaryWindow,
};

pub mod prelude {
    pub use crate::{FixedViewport, FixedViewportPlugin, FixedViewportSystems};
}

/// Contains the fixed viewport functionality within the assigned schedule.
#[derive(SystemSet, Hash, Debug, Clone, PartialEq, Eq)]
pub struct FixedViewportSystems;

/// Attach this to a camera to enforce a fixed viewport of a given aspect ratio.
#[derive(Component, Clone, Default)]
pub struct FixedViewport {
    pub aspect_ratio: f32,
}

/// Adds fixed viewport functionality to the app.
pub struct FixedViewportPlugin;

impl Plugin for FixedViewportPlugin {
    fn build(&self, app: &mut App) {
        // The pre-update schedule is selected as it is more likely that something within update or post-update will want to
        // adjust something based on the viewport; for example some complex UI behaviour.
        // It is also assumed that window changes happen prior to anything in first/pre-update/etc.
        app.add_systems(PreUpdate, update_viewports.in_set(FixedViewportSystems));
    }
}

/// Updates viewport according to the fixed viewport configuration.
fn update_viewports(
    mut camera_query: Query<(Ref<FixedViewport>, Ref<RenderTarget>, &mut Camera)>,
    window_query: Query<(Ref<Window>, Has<PrimaryWindow>)>,
) {
    for (fixed_viewport, render_target, mut camera) in camera_query.iter_mut() {
        // find the window that the camera renders to
        let window = match render_target.into_inner() {
            RenderTarget::Window(window_ref) => match window_ref {
                bevy::window::WindowRef::Primary => {
                    match window_query.iter().find(|(_, primary)| *primary) {
                        Some((window, _)) => window,
                        None => continue,
                    }
                }
                bevy::window::WindowRef::Entity(entity) => match window_query.get(*entity) {
                    Ok((window, ..)) => window,
                    Err(_) => continue,
                },
            },
            _ => continue, // non-window targets not supported
        };

        // skip if nothing changed
        if !window.is_changed() && !fixed_viewport.is_changed() && !render_target.is_changed() {
            continue;
        }

        let window_width = window.physical_width() as f32;
        let window_height = window.physical_height() as f32;
        let mut width = window_width;
        let mut height = window_height;
        let ratio = window_width / window_height;
        if ratio > fixed_viewport.aspect_ratio {
            width = height * fixed_viewport.aspect_ratio;
        } else {
            height = width / fixed_viewport.aspect_ratio;
        }

        let x = window_width / 2.0 - width / 2.0;
        let y = window_height / 2.0 - height / 2.0;

        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(x as u32, y as u32),
            physical_size: UVec2::new(width as u32, height as u32),
            ..default()
        });
    }
}
