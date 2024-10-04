use bevy::{
    prelude::*,
    render::camera::{RenderTarget, Viewport},
    window::{PrimaryWindow, WindowRef, WindowResized, WindowScaleFactorChanged},
};
use itertools::Itertools;

/// Contains functionality for fitting a fixed viewport aspect ratio to the window.
pub struct FixedViewportPlugin;

impl Plugin for FixedViewportPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SyncEvent>().add_systems(
            PostUpdate,
            (
                (emit_camera_sync_event, emit_window_sync_event),
                sync_viewport,
            )
                .chain(),
        );
    }
}

/// The things that can cause a sync to happen.
#[derive(Event)]
enum SyncEvent {
    /// The camera's fixed viewport has changed.
    Camera(Entity),
    // The window was resized or the scale factor changed.
    Window(Entity),
}

/// Attach this to a camera make the viewport fit the available window space with a fixed aspect ratio.
#[derive(Component)]
pub struct FixedViewport {
    pub aspect_ratio: f32,
}

/// Emits a sync event when the camera's fixed viewport changes.
fn emit_camera_sync_event(
    camera_query: Query<Entity, Changed<FixedViewport>>,
    mut sync_events: EventWriter<SyncEvent>,
) {
    for entity in camera_query.iter() {
        sync_events.send(SyncEvent::Camera(entity));
    }
}

/// Emits a sync event when the window's size or scale factor changes.
fn emit_window_sync_event(
    mut resize_events: EventReader<WindowResized>,
    mut scale_factor_events: EventReader<WindowScaleFactorChanged>,
    mut sync_events: EventWriter<SyncEvent>,
) {
    for event in resize_events.read() {
        sync_events.send(SyncEvent::Window(event.window));
    }
    for event in scale_factor_events.read() {
        sync_events.send(SyncEvent::Window(event.window));
    }
}

/// Synchronizes camera's viewport with the window size according to the fixed viewport.
fn sync_viewport(
    mut sync_events: EventReader<SyncEvent>,
    mut camera_query: Query<(&FixedViewport, &mut Camera)>,
    window_query: Query<(&Window, Option<&PrimaryWindow>)>,
) {
    for event in sync_events.read() {
        // resolve the required data for the syncing
        let (window, mut cameras) = match event {
            // the event came from the camera, find the matching window
            SyncEvent::Camera(entity) => match camera_query.get_mut(*entity) {
                Ok((fixed_viewport, camera)) => (
                    match &camera.target {
                        RenderTarget::Window(window_ref) => match window_ref {
                            // if more than one primary window (extremely likely), we cannot continue
                            WindowRef::Primary => match window_query
                                .iter()
                                .filter(|(_, primary_window)| primary_window.is_some())
                                .exactly_one()
                            {
                                Ok((window, _)) => window,
                                Err(_) => continue,
                            },
                            WindowRef::Entity(entity) => match window_query.get(*entity) {
                                Ok((window, _)) => window,
                                Err(_) => continue,
                            },
                        },
                        _ => continue,
                    },
                    vec![(fixed_viewport, camera)],
                ),
                Err(_) => continue,
            },
            // the event came from the window, find the matching camera
            SyncEvent::Window(entity) => {
                // get the window data first
                let (window, primary_window) = match window_query.get(*entity) {
                    Ok(item) => item,
                    Err(_) => continue,
                };

                // find all matching cameras
                (
                    window,
                    camera_query
                        .iter_mut()
                        .filter_map(|(fixed_viewport, camera)| match camera.target {
                            RenderTarget::Window(window_ref) => match window_ref {
                                WindowRef::Primary => match primary_window {
                                    Some(_) => Some((fixed_viewport, camera)),
                                    None => None,
                                },
                                WindowRef::Entity(ref_entity) => match ref_entity == *entity {
                                    true => Some((fixed_viewport, camera)),
                                    false => None,
                                },
                            },
                            _ => None,
                        })
                        .collect(),
                )
            }
        };

        for (fixed_viewport, camera) in cameras.iter_mut() {
            // get the required data
            let window_width = window.physical_width() as f32;
            let window_height = window.physical_height() as f32;
            let window_ratio = window_width / window_height;
            let mut viewport_width = window_width;
            let mut viewport_height = window_height;
            let mut viewport_x = 0f32;
            let mut viewport_y = 0f32;

            // determine the best fit for the given aspect ratio
            if window_ratio > fixed_viewport.aspect_ratio {
                viewport_width = viewport_height * fixed_viewport.aspect_ratio;
                viewport_x = window_width / 2. - viewport_width / 2.;
            } else {
                viewport_height = viewport_width / fixed_viewport.aspect_ratio;
                viewport_y = window_height / 2. - viewport_height / 2.;
            }

            // update the viewport accordingly
            camera.viewport = Some(Viewport {
                physical_position: UVec2::new(viewport_x as u32, viewport_y as u32),
                physical_size: UVec2::new(viewport_width as u32, viewport_height as u32),
                ..default()
            });
        }
    }
}
