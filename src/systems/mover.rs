use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::components::{CreatureTag, MoveSpeed};

#[derive(SystemDesc)]
pub struct MoverSystem;

impl<'s> System<'s> for MoverSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, CreatureTag>,
        ReadStorage<'s, MoveSpeed>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, creatures, move_speeds, delta_time): Self::SystemData) {

        for (_, transform, move_speed) in (&creatures, &mut transforms, &move_speeds).join() {
            let move_distance = delta_time.fixed_seconds() * move_speed.speed;
            //println!("move_distance: {}", move_distance);
            

            transform.prepend_translation_x(move_distance); 

            transform.prepend_translation_y(move_distance); 
        }
    }
}