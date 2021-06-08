use amethyst::core::{Transform};
use amethyst::core::timing::Time;
use amethyst::ecs::{System, World, SystemBuilder};
use amethyst::prelude::*;

use crate::orbital::{MovingObject};
// Legion RFC is the thing to read 
// https://github.com/amethyst/rfcs/issues/22

pub struct VelocitySystem;

impl System for VelocitySystem {
    fn build(self) -> Box<dyn ParallelRunnable> {
        Box::new(
        SystemBuilder::new("VelocitySystem")

            .with_query(<(&MovingObject, &mut Transform)>::query())
            .write_component::<MovingObject>()
            .write_component::<Transform>()
            .read_resource::<Time>()
            .build(move | _commands, world, time, query_objects | {
                for (moving_object, local) in query_objects.iter_mut(world) {
                    local.prepend_translation_x(moving_object.velocity[0] * time.delta_seconds());
                    local.prepend_translation_y(moving_object.velocity[1] * time.delta_seconds());
                }
            })
        )
    }
}
