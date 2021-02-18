use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage, Entities, Write};

use crate::components::{CreatureTag, MoveSpeed};
use crate::resources::SpawnCounter;

#[derive(SystemDesc)]
pub struct SpawnerSystem {
    pub interval_count: u32,
}

impl<'s> System<'s> for SpawnerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, MoveSpeed>,
        WriteStorage<'s, CreatureTag>,
        Read<'s, SpawnCounter>,
        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, mut move_speeds, mut creature_tags, spawn_counter, entities): Self::SystemData) {
        self.interval_count += 1;

        if self.interval_count > 1000 {

            
            let diff = spawn_counter.cells - (&creature_tags, &mut transforms).join().count() as u32;

            for _ in 0..diff {
                entities.build_entity()
                    .with(Transform::default(), &mut transforms)
                    .with(CreatureTag {}, &mut creature_tags)
                    .with(MoveSpeed::new(2.0), &mut move_speeds)
                    .build();
            }

            self.interval_count = 0;
        }
    }
}
