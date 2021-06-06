use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::orbital::{Planet, MovingObject};

#[derive(SystemDesc)]
pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Planet>,
        WriteStorage<'s, MovingObject>
    );

    fn run(&mut self, (mut transforms, planets, mut ships): Self::SystemData) {

    }
}