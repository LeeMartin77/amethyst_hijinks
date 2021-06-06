use amethyst::core::{Transform, SystemDesc};
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::orbital::{MovingObject};

#[derive(SystemDesc)]
pub struct VelocitySystem;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, MovingObject>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut moving_objects, time): Self::SystemData) {
        for (moving_object, local) in (&moving_objects, &mut transforms).join() {
            local.prepend_translation_x(moving_object.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(moving_object.velocity[1] * time.delta_seconds());
        }
    }
}