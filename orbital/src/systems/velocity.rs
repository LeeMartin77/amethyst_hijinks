use amethyst::core::{Transform};
use amethyst::core::timing::Time;
use amethyst::ecs::{System, World, SystemBuilder};

use crate::orbital::{MovingObject};
// Legion RFC is the thing to read 
// https://github.com/amethyst/rfcs/issues/22
fn build_velocity_system(
    world: &mut amethyst::ecs::World,
) -> Box<dyn amethyst::ecs::Schedulable> 
{
    SystemBuilder::<()>::new("VelocitySystem")
        .write_component::<MovingObject>()
        .write_component::<Transform>()
        .read_resource::<Time>()
        .build(move | commands, world, (moving_objects, transforms, time), query | {
            for (moving_object, local) in (&moving_objects, &mut transforms).join() {
                local.prepend_translation_x(moving_object.velocity[0] * time.delta_seconds());
                local.prepend_translation_y(moving_object.velocity[1] * time.delta_seconds());
            }
        })
}