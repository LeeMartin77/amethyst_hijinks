use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::orbital::{Planet};

#[derive(SystemDesc)]
pub struct PlanetSystem;

impl<'s> System<'s> for PlanetSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Planet>
    );

    fn run(&mut self, (mut transforms, planets): Self::SystemData) {

    }
}