use amethyst::core::timing::Time;
use amethyst::ecs::{System, SystemBuilder};
use amethyst::prelude::*;
use amethyst::renderer::SpriteRender;

use crate::entities::{player_character::PlayerCharacter};
// Legion RFC is the thing to read 
// https://github.com/amethyst/rfcs/issues/22

pub struct PlayerAnimationSystem {
    idle_frame: u8,
    idle_time: f32
}

impl PlayerAnimationSystem {
    pub fn new() -> PlayerAnimationSystem {
        PlayerAnimationSystem {
            idle_frame: 0,
            idle_time: 0.0
        }
    }
}

impl System for PlayerAnimationSystem {
    fn build(mut self) -> Box<dyn ParallelRunnable> {
        Box::new(
        SystemBuilder::new("PlayerAnimationSystem")
            .with_query(<(&PlayerCharacter, &mut SpriteRender)>::query())
            .read_component::<PlayerCharacter>()
            .write_component::<SpriteRender>()
            .read_resource::<Time>()
            .build(move | _commands, world, time, query_objects | {
                for (_player_char, mut render) in query_objects.iter_mut(world) {
                    if self.idle_time > 0.4 {
                        self.idle_time = 0.0;
                        self.idle_frame = self.idle_frame + 1;
                        if self.idle_frame > 3 {
                            self.idle_frame = 0;
                        }
                        render.sprite_number = self.idle_frame.into();
                    }
                    else
                    {
                        self.idle_time = self.idle_time + time.delta_seconds();
                    }
                }
            })
        )
    }
}
