use amethyst::core::timing::Time;
use amethyst::ecs::{System, SystemBuilder};
use amethyst::prelude::*;
use amethyst::renderer::SpriteRender;

use crate::entities::{player_character::PlayerCharacter};
// Legion RFC is the thing to read 
// https://github.com/amethyst/rfcs/issues/22

pub struct PlayerAnimationSystem {
    idle_frames: [u8; 4],
    idle_frame_index: u8,
    idle_time: f32
}

impl PlayerAnimationSystem {
    pub fn new() -> PlayerAnimationSystem {
        PlayerAnimationSystem {
            idle_frames: [40, 41, 42, 43],
            idle_frame_index: 0,
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
                        self.idle_frame_index = self.idle_frame_index + 1;
                        if self.idle_frame_index as usize > self.idle_frames.len() - 1 {
                            self.idle_frame_index = 0;
                        }
                        render.sprite_number = self.idle_frames[self.idle_frame_index as usize].into();
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
