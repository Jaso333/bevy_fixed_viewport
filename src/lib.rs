use std::collections::HashSet;

use bevy_app::prelude::*;
use bevy_camera::{RenderTarget, Viewport, prelude::*};
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_window::{PrimaryWindow, WindowRef, WindowResized, prelude::*};

/// Contains the fixed viewport functionality within the assigned schedule.
#[derive(SystemSet, Hash, Debug, Clone, PartialEq, Eq)]
pub struct FixedViewportSystems;

/// Attach this to a camera to enforce a fixed viewport of a given aspect ratio.
#[derive(Component, Clone, Default)]
pub struct FixedViewport {
    /// The aspect ratio to maintain (width / height).
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
    window_query: Query<(Entity, &Window, Option<Ref<PrimaryWindow>>)>,
    mut messages: MessageReader<WindowResized>,
) {
    let changed_windows = messages.read().map(|m| m.window).collect::<HashSet<_>>();

    for (fixed_viewport, render_target, mut camera) in camera_query.iter_mut() {
        // find the window that the camera renders to
        let (window_entity, window, mut window_changed) = match render_target.into_inner() {
            RenderTarget::Window(window_ref) => match window_ref {
                WindowRef::Primary => {
                    match window_query
                        .iter()
                        .find(|(_, _, primary)| primary.is_some())
                    {
                        // account for the possibility that the window has just been made primary
                        Some((window_entity, window, primary)) => {
                            (window_entity, window, primary.unwrap().is_added())
                        }
                        None => continue,
                    }
                }
                WindowRef::Entity(window_entity) => match window_query.get(*window_entity) {
                    Ok((window_entity, window, ..)) => (window_entity, window, false),
                    Err(_) => continue,
                },
            },
            _ => continue, // non-window targets not supported
        };

        // check the events about this window
        window_changed = window_changed || changed_windows.contains(&window_entity);

        // skip if nothing changed
        if !window_changed && !fixed_viewport.is_changed() && !render_target.is_changed() {
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
            ..Default::default()
        });
    }
}
