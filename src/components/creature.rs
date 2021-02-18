use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
    ecs::{Component, DenseVecStorage},
    assets::Handle,
};
use rand::Rng;

pub struct CreatureTag {
} 

impl Component for CreatureTag {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_creatures(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut rng = rand::thread_rng();
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    for _ in 0..100 {
        let mut transform = Transform::default();

        transform.set_translation_xyz(rng.gen_range(0.0..1800.0), rng.gen_range(-0.0..900.0), 0.0);

        world
            .create_entity()
            .with(CreatureTag {})
            .with(MoveSpeed::new(rng.gen_range(0.0..10.0)))
            .with(transform)
            .with(sprite_render.clone())
            .build();
    }
}

pub struct MoveSpeed {
    pub speed: f32,
}

impl MoveSpeed {
    pub fn new(speed: f32) -> MoveSpeed {
        MoveSpeed {
            speed,
        }
    }
}

impl Component for MoveSpeed {
    type Storage = DenseVecStorage<Self>;
}

pub struct ViewDistance {
    pub distance: f32,
}

impl ViewDistance {
    pub fn new(distance: f32) -> ViewDistance {
        ViewDistance {
            distance
        }
    }
}

impl Component for ViewDistance {
    type Storage = DenseVecStorage<Self>;
}

pub struct Hunger {
    pub max: f32,
    pub current: f32,
}

impl Hunger {
    pub fn new(max: f32) -> Hunger {
        Hunger {
            max,
            current: max,
        }
    }
}

impl Component for Hunger {
    type Storage = DenseVecStorage<Self>;
}

pub struct Experience {
    pub value: f32,
}

impl Experience {
    pub fn new() -> Experience {
        Experience {
            value: 0.0,
        }
    }
}

impl Component for Experience {
    type Storage = DenseVecStorage<Self>;
}

pub struct MutationChance {
    pub chance: f32,
}

impl MutationChance {
    pub fn new(chance: f32) -> MutationChance {
        MutationChance {
            chance
        }
    }
}

impl Component for MutationChance {
    type Storage = DenseVecStorage<Self>;
}

pub struct PickupRange {
    pub distance: f32,
}

impl PickupRange {
    pub fn new(distance: f32) -> PickupRange {
        PickupRange {
            distance,
        }
    } 
}

impl Component for PickupRange {
    type Storage = DenseVecStorage<Self>;
}

pub struct Age {
    pub value: f32,
}

impl Age {
    pub fn new(value: f32) -> Age {
        Age {
            value
        }
    }
}

impl Component for Age {
    type Storage = DenseVecStorage<Self>;
}