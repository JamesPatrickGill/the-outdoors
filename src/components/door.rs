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

use crate::outdoors::{ARENA_HEIGHT, ARENA_WIDTH, X_SCALING, Y_SCALING};

#[derive(Debug, PartialEq, Eq)]
pub enum Entrance {
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct Door {
    pub entrance: Entrance,
    pub width: f32,
    pub height: f32,
}

impl Door {
    pub fn new() -> Door {
        Door {
            entrance: Entrance::RIGHT,
            width: 8.0,
            height: 64.0,
        }
    }
}

impl Component for Door {
    type Storage = DenseVecStorage<Self>;
}

fn load_door_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/doors.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/doors.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub fn initialise_door(world: &mut World) {
    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: load_door_sprite_sheet(world).clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    let mut local_transform = Transform::default();

    local_transform
        .set_translation_xyz(ARENA_WIDTH / 2.0 + 40.0, ARENA_HEIGHT * 0.4 + 32.0, 0.0)
        .set_scale(Vector3::new(X_SCALING, Y_SCALING, 1.0));

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Door::new())
        .with(local_transform)
        .build();
}
