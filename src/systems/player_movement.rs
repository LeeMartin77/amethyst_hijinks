use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::{System, SystemBuilder};
use amethyst::input::{get_input_axis_simple, InputHandler};
use amethyst::prelude::*;
use amethyst::renderer::SpriteRender;

use crate::entities::{player_character::PlayerCharacter};
// Legion RFC is the thing to read 
// https://github.com/amethyst/rfcs/issues/22

pub struct PlayerMovementSystem {
}

impl PlayerMovementSystem {
    pub fn new() -> PlayerMovementSystem {
        PlayerMovementSystem {
        }
    }
}

impl System for PlayerMovementSystem {
    fn build(mut self) -> Box<dyn ParallelRunnable> {
        Box::new(
        SystemBuilder::new("PlayerMovementSystem")
            .with_query(<(&PlayerCharacter, &mut Transform)>::query())
            .read_resource::<InputHandler>()
            .read_resource::<Time>()
            .build(move | _commands, world, (input, time), query_objects | {
                for (_player_char, transform) in query_objects.iter_mut(world) {
                    let movement = get_input_axis_simple(&Some("left_right".into()), input);
                    let movement_scale = 60.0;
                    let scaled_movement = movement_scale * movement * time.delta_seconds();
                    let player_y = transform.translation().x;
                    transform.set_translation_x(player_y + scaled_movement);
                }
            })
        )
    }
}
