use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture },
    window::ScreenDimensions,
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
    core::timing::Time
};

#[derive(Default)]
pub struct Orbital {
}

impl SimpleState for Orbital {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    let screen_width = world.read_resource::<ScreenDimensions>().width();
    let screen_height = world.read_resource::<ScreenDimensions>().height();
    transform.set_translation_xyz(screen_width.clone() * 0.5, screen_height.clone() * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(screen_width, screen_height))
        .with(transform)
        .build();
}
