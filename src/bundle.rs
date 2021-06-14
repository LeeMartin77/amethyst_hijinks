use amethyst::{
    core::ecs::SystemBundle,
    ecs::{DispatcherBuilder, Resources, World},
    error::Error,
};


use crate::systems::{
    player_animation::PlayerAnimationSystem,
    player_movement::PlayerMovementSystem
};

/// A bundle is a convenient way to initialize related resources, components and systems in a
/// world.
pub struct RogueliteBundle;

impl SystemBundle for RogueliteBundle {
    fn load(
        &mut self,
        _world: &mut World,
        _resources: &mut Resources,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), Error> {
        builder
            .add_system(PlayerMovementSystem::new())
            .add_system(PlayerAnimationSystem::new())
            .flush();
        Ok(())
    }
}