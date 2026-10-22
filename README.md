# Design

It is common, mainly in 2D games, that you want the bounds of the world to be consistent regardless of the physical window size. This prevents off-screen visuals being exposed, and prevents the player seeing more than they should.

This library provides a `FixedViewport` component that you can add to `Camera` entities which sets the viewport of the camera to be of the largest fixed size for a given aspect ratio. This results in a "letterboxing" effect.

# Usage

Add the crate:

```toml
bevy_fixed_viewport = "0.1.0"
```

Add the plugin:

```rust
app.add_plugins(FixedViewportPlugin);
```

Use the component:

```rust
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
```

It is recommended to also use `ScalingMode::Fixed` for the camera projection scaling mode to ensure the world size is also fixed.

# License

bevy_fixed_viewport is free and open source. All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.