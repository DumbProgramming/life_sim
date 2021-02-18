use amethyst::ecs::{Component, DenseVecStorage};

pub struct Food {
    pub nutrients: f32,
}

impl Food {
    pub fn new(nutrients: f32) -> Food {
        Food {
            nutrients
        }
    }
}

impl Component for Food {
    type Storage = DenseVecStorage<Self>;
}