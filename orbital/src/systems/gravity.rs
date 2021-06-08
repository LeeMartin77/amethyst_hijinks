use amethyst::core::{Transform};
use amethyst::core::timing::Time;
use amethyst::ecs::{System, World, SystemBuilder};
use amethyst::prelude::*;
use amethyst::core::math::{distance, normalize, Point3};


use crate::orbital::{Planet, MovingObject};

pub struct GravitySystem;

impl System for GravitySystem {
    fn build(self) -> Box<dyn ParallelRunnable> {
        return Box::new(
            SystemBuilder::new("GravitySystem")
            .with_query(<(&mut MovingObject, &Transform)>::query())
            .with_query(<(&Planet, &Transform)>::query())
            .write_component::<MovingObject>()
            .read_component::<Transform>()
            .read_component::<Planet>()
            .read_resource::<Time>()
            .build(move | _commands, world, time, moving_objects, planets | {
                let on_planet = false;
                for (moving_object, object_local) in moving_objects.iter_mut(world) {
                    for (planet, planet_local) in planets.iter_mut(world){
                        // if distance(moving_local.translation(), planet_local.translation()) < planet.radius {
                        //     //This will be a gameover at some point
                        //     moving_object.velocity = [0.0, 0.0];
                        //     on_planet = true;
                        //     break;
                        // }
                        apply_gravity(&time, &moving_object, &object_local, planet, planet_local)
                    }
                    if on_planet { break; }
                }
            })
        )
    }
}


fn apply_gravity(time: &Time, moving_object: &mut MovingObject, object_local: &Transform, planet: Planet, planet_local: Transform){
    //We should definitely not be calcing this constantly.
    //https://en.wikipedia.org/wiki/Gravitational_constant
    let gravitational_constant = 6.674; //* Math.Pow(10d, -11d);
    let force = gravitational_constant * ((moving_object.mass * planet.mass) / distance(object_local.translation(), planet_local.translation())); //Vector2.Distance(mobileEntity.Position, staticEntity.Position));
    //F=MA
    //A = F/M
    let acceleration = force / moving_object.mass;
    //A = distance/time
    //distance = A*time
    let distance = acceleration * time.delta_seconds();
    let normalised_vector = normalize(planet_local.translation() - object_local.translation());
    moving_object.velocity = [moving_object.velocity[0] + (normalised_vector.x * distance), moving_object.velocity[1] + (normalised_vector.y * distance)]
}