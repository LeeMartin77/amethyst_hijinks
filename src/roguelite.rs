use amethyst::{
    assets::{DefaultLoader, Handle, Loader, ProcessingQueue},
    core::transform::Transform,
    prelude::*,
    renderer::{sprite::Sprites, Camera, SpriteRender, SpriteSheet},
    window::ScreenDimensions
};

use crate::entities::{ 
    player_character::PlayerCharacter
};

#[derive(Default)]
pub struct Roguelite {
}

impl SimpleState for Roguelite {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let StateData {
            world, resources, ..
        } = data;
        initialise_player(world, resources);
        initialise_camera(world, resources);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData>) -> SimpleTrans {
        Trans::None
    }
}

fn initialise_camera(world: &mut World, resources: &mut Resources) {
    let mut transform = Transform::default();
    let screen_width = resources.get::<ScreenDimensions>().unwrap().width();
    let screen_height = resources.get::<ScreenDimensions>().unwrap().height();
    transform.set_translation_xyz(0.0, 0.0, 1.0);
    let scaling = 0.25;
    world.push((Camera::standard_2d(screen_width * scaling, screen_height * scaling), transform));
}


fn initialise_player(world: &mut World, resources: &mut Resources) {
    let mut player_transform = Transform::default();

    player_transform.set_translation_xyz(0.0, 0.0, 0.0);

    let player_char = PlayerCharacter::new(resources);
    let start_sprite = SpriteRender::new(player_char.spritesheet.clone(), 40);

    world.push((player_char, player_transform, start_sprite));
}