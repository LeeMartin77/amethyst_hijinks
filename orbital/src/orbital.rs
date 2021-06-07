use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, Texture },
    window::ScreenDimensions,
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
    core::timing::Time
};

#[derive(Default)]
pub struct Orbital {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}


pub struct Planet {
    pub radius: f32,
    pub mass: f32,
}

impl Planet {
    fn new(radius: f32, mass: f32) -> Planet {
        Planet {
            radius,
            mass
        }
    }
}

pub struct MovingObject {
    pub velocity: [f32; 2],
    pub mass: f32
}

impl MovingObject {
    fn new(velocity: [f32; 2], mass: f32) -> MovingObject {
        MovingObject { 
            velocity,
            mass
        }
    }
}

impl SimpleState for Orbital {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let world = data.world;
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        initialise_planet(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_ship(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData>) -> SimpleTrans {
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    let screen_width = world.read_resource::<ScreenDimensions>().width();
    let screen_height = world.read_resource::<ScreenDimensions>().height();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world.push((Camera::standard_2d(screen_width * 4.0, screen_height * 4.0), transform));
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/rough_sprites.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/rough_sprites.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialise_planet(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut planet_transform = Transform::default();
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    planet_transform.set_translation_xyz(0.0, 0.0, 0.0);

    world.push((Planet::new(528.0 * 0.5, 1000.0), sprite_render.clone(), planet_transform));
}


fn initialise_ship(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut ship_transform = Transform::default();
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    ship_transform.set_translation_xyz(0.0, 720.0, 0.0);

    world.push((MovingObject::new([0.5, 0.5], 0.0005), ship_transform, sprite_render.clone()));
}