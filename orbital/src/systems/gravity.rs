use amethyst::core::{Transform, SystemDesc };
use amethyst::core::math::{ Vector3, distance};
use amethyst::core::alga::linear::EuclideanSpace;
use amethyst::derive::SystemDesc;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::orbital::{Planet, MovingObject};

#[derive(SystemDesc)]
pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Planet>,
        WriteStorage<'s, MovingObject>,
        Read<'s, Time>,
    );

    fn run(&mut self, (transforms, planets, mut moving_objects, time): Self::SystemData) {
        for (moving_object, moving_local) in (&mut moving_objects, & transforms).join() {
            for (planet, planet_local) in (&planets, &transforms).join() {

                //We should definitely not be calcing this constantly.
                //https://en.wikipedia.org/wiki/Gravitational_constant
                let gravitational_constant = 6.674; //* Math.Pow(10d, -11d);
                let force = gravitational_constant * ((moving_object.mass * planet.mass) / distance(moving_local.translation(), planet_local.translation())); //Vector2.Distance(mobileEntity.Position, staticEntity.Position));
                //F=MA
                //A = F/M
                //var acceleration = force / mobileEntity.Mass;
                //A = distance/time
                //distance = A*time
                //var distance = (float)(acceleration * gameTime.ElapsedGameTime.TotalSeconds);
                //var normalisedVector = Vector2.Normalize(staticEntity.Position - mobileEntity.Position);
                //return new Vector2(normalisedVector.X * distance, normalisedVector.Y * distance);
                if distance(moving_local.translation(), planet_local.translation()) < planet.radius {
                    //If the object is in/on the planet, it needs to stop moving
                    //This will be a gameover at some point
                    //This probably won't actually work but is a statement of intent
                    moving_object.velocity = [0.0, 0.0]
                }
            }
        }
    }
}