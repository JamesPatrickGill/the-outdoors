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

use crate::outdoors::{Plane, ARENA_HEIGHT, ARENA_WIDTH, X_SCALING, Y_SCALING};

pub const GROUND_HEIGHT: f32 = 2.0;
pub const GROUND_WIDTH: f32 = 32.0;

#[derive(Debug)]
pub struct Ground {
    pub width: f32,
    pub height: f32,
    pub plane: Plane,
}

impl Ground {
    pub fn new(plane: Plane) -> Ground {
        Ground {
            width: GROUND_HEIGHT,
            height: GROUND_WIDTH,
            plane: plane,
        }
    }
}

impl Component for Ground {
    type Storage = DenseVecStorage<Self>;
}

fn load_ground_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/ground1.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/ground1.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub fn initialise_ground(world: &mut World) {
    // Assign the sprites for the paddles

    let sprite_sheet_handle = load_ground_sprite_sheet(world);
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };
    let sprite_render_1 = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1, // paddle is the first sprite in the sprite_sheet
    };

    let mut local_transform = Transform::default();
    let mut local_transform_1 = Transform::default();

    local_transform
        .set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT * 0.4, 0.0)
        .set_scale(Vector3::new(ARENA_WIDTH / GROUND_WIDTH, Y_SCALING, 1.0));

    local_transform_1
        .set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT * 0.4, 1.0)
        .set_scale(Vector3::new(ARENA_WIDTH / GROUND_WIDTH, Y_SCALING, 1.0));

    world
        .create_entity()
        .with(sprite_render_1)
        .with(Ground::new(Plane::ZERO))
        .with(local_transform)
        .build();

    world
        .create_entity()
        .with(sprite_render)
        .with(Ground::new(Plane::ONE))
        .with(local_transform_1)
        .build();
}
