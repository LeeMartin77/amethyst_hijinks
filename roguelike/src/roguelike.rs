use amethyst::{
    assets::{DefaultLoader, Handle, Loader, ProcessingQueue},
    core::transform::Transform,
    prelude::*,
    renderer::{sprite::Sprites, Camera, SpriteRender, SpriteSheet},
    window::ScreenDimensions
};

#[derive(Default)]
pub struct Roguelike {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Roguelike {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let StateData {
            world, resources, ..
        } = data;
        self.sprite_sheet_handle.replace(load_sprite_sheet(resources));
        initialise_camera(world, resources);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData>) -> SimpleTrans {
        Trans::None
    }
}

fn initialise_camera(world: &mut World, resources: &mut Resources) {
    let mut transform = Transform::default();
    let screen_width = resources.get::<ScreenDimensions>().unwrap().width();
    let screen_height = resources.get::<ScreenDimensions>().unwrap().height();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world.push((Camera::standard_2d(screen_width * 4.0, screen_height * 4.0), transform));
}

fn load_sprite_sheet(resources: &mut Resources) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = resources.get::<DefaultLoader>().unwrap();
        loader.load("texture/rough_sprites.png")
    };

    let loader = resources.get::<DefaultLoader>().unwrap();
    let sprites: Handle<Sprites> = loader.load(
        "texture/rough_sprites.ron", // Here we load the associated ron file
    );
    let sheet = SpriteSheet {
        texture: texture_handle,
        sprites,
    };
    let q = resources.get::<ProcessingQueue<SpriteSheet>>().unwrap();
    loader.load_from_data(sheet, (), &q)
}