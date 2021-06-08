// use amethyst::core::{Transform};
// use amethyst::core::timing::Time;
// use amethyst::ecs::{System, World, SystemBuilder};
// use amethyst::prelude::*;


// use crate::orbital::{Planet, MovingObject};

// pub struct GravitySystem;

// impl System for GravitySystem {
//     fn build(self) -> Box<dyn ParallelRunnable> {
//         return Box::new(
//             SystemBuilder::new("GravitySystem")
//             .with_query(<(&MovingObject, &mut Transform)>::query())
//         )


//                 //We should definitely not be calcing this constantly.
//                 //https://en.wikipedia.org/wiki/Gravitational_constant
//                 //let gravitational_constant = 6.674; //* Math.Pow(10d, -11d);
//                 //let force = gravitational_constant * ((moving_object.mass * planet.mass) / distance(moving_local.translation(), planet_local.translation())); //Vector2.Distance(mobileEntity.Position, staticEntity.Position));
//                 //F=MA
//                 //A = F/M
//                 //var acceleration = force / mobileEntity.Mass;
//                 //A = distance/time
//                 //distance = A*time
//                 //var distance = (float)(acceleration * gameTime.ElapsedGameTime.TotalSeconds);
//                 //var normalisedVector = Vector2.Normalize(staticEntity.Position - mobileEntity.Position);
//                 //return new Vector2(normalisedVector.X * distance, normalisedVector.Y * distance);
//                 //if distance(moving_local.translation(), planet_local.translation()) < planet.radius {
//                     //If the object is in/on the planet, it needs to stop moving
//                     //This will be a gameover at some point
//                     //This probably won't actually work but is a statement of intent
//                 //    moving_object.velocity = [0.0, 0.0]
//                 //}
//     }
// }