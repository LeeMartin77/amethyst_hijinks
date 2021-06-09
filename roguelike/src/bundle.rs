use amethyst::{
    core::ecs::SystemBundle,
    ecs::{DispatcherBuilder, Resources, World},
    error::Error,
};


use crate::systems::{
    
};

/// A bundle is a convenient way to initialize related resources, components and systems in a
/// world.
pub struct RoguelikeBundle;

impl SystemBundle for RoguelikeBundle {
    fn load(
        &mut self,
        _world: &mut World,
        _resources: &mut Resources,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), Error> {
        builder
            //.add_system(GravitySystem)
            .flush();
        Ok(())
    }
}