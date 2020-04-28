use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::{Transform, TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        Camera, ImageFormat, RenderingBundle, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture,
    },
    utils::application_root_dir,
};

mod components;
mod systems;

mod outdoors;
use crate::outdoors::Outdoors;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(
            systems::MovePlayerSystem,
            "move_player_system",
            &["input_system"],
        )
        .with(
            systems::DoorCollisions,
            "door_collision_system",
            &["move_player_system"],
        );

    let mut game = Application::new(assets_dir, Outdoors::default(), game_data)?;
    game.run();

    Ok(())
}
