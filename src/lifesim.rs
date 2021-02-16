use amethyst::{
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    ecs::{Component, DenseVecStorage},
    assets::{AssetStorage, Loader, Handle},
};
use rand::Rng;

use log::info;

/// A dummy game state that shows 3 sprites.
pub struct LifeSim;

impl SimpleState for LifeSim {
   
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Creature>();
        initialise_creatures(world, sprite_sheet_handle);
        // Place the camera
        init_camera(world, &dimensions);
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }
        }

        // Keep going
        Trans::None
    }
}

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

pub struct Creature {
    pub width: f32,
    pub height: f32,
} 

impl Creature {
    fn new(width: f32, height: f32) -> Creature {
        Creature {
            width,
            height,
        }
    }
}

impl Component for Creature {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_creatures(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut rng = rand::thread_rng();
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    for _ in 0..5000 {
        let mut transform = Transform::default();

        let y = rng.gen_range(-1000.0..1000.0);
        let x = rng.gen_range(-1000.0..1000.0);

        transform.set_translation_xyz(x, y, 0.0);

        world
            .create_entity()
            .with(Creature::new(20.0, 20.0))
            .with(transform)
            .with(sprite_render.clone())
            .build();
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/sheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/sheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}