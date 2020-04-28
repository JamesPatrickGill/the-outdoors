use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::components::{
    initialise_door, initialise_ground, initialise_player, Door, Ground, Player,
};

pub const ARENA_HEIGHT: f32 = 720.0;
pub const ARENA_WIDTH: f32 = 960.0;

pub const Y_SCALING: f32 = 1.0;
pub const X_SCALING: f32 = 1.0;

#[derive(Debug)]
pub enum Plane {
    ZERO,
    ONE,
    TWO,
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 4.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

#[derive(Default)]
pub struct Outdoors {}

impl SimpleState for Outdoors {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Player>();
        world.register::<Ground>();
        world.register::<Door>();

        initialise_camera(world);
        initialise_player(world);
        initialise_ground(world);
        initialise_door(world);
    }
}
