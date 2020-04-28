use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{
        palette::Srgba, resources::Tint, Camera, ImageFormat, SpriteRender, SpriteSheet,
        SpriteSheetFormat, Texture,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::outdoors::{ARENA_HEIGHT, ARENA_WIDTH, X_SCALING, Y_SCALING};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT,
    NONE,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerState {
    IDLE,
    WALKING,
}

#[derive(Debug)]
pub struct Player {
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
    pub animation_counter: f32,
    pub facing: Direction,
    pub state: PlayerState,
    pub x_collision: Direction,
}

impl Player {
    pub fn new() -> Player {
        Player {
            width: 28.0,
            height: 64.0,
            velocity: [0.0, 0.0],
            animation_counter: 0.07,
            facing: Direction::RIGHT,
            state: PlayerState::IDLE,
            x_collision: Direction::NONE,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

fn load_player_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/player-walk-white.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/player-walk-white.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

/// Initialises one paddle on the left, and one paddle on the right.
pub fn initialise_player(world: &mut World) {
    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: load_player_sprite_sheet(world),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    let mut local_transform = Transform::default();

    local_transform
        .set_translation_xyz(ARENA_WIDTH / 2.0 - 200.0, ARENA_HEIGHT * 0.4 + 32.0, 0.0)
        .set_scale(Vector3::new(X_SCALING, Y_SCALING, 1.0));

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Player::new())
        .with(local_transform)
        .build();
}
