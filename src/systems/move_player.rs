use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{palette::Srgba, resources::Tint, SpriteRender},
};

use crate::components::{Direction, Player};

#[derive(SystemDesc)]
pub struct MovePlayerSystem;

impl<'s> System<'s> for MovePlayerSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut players, mut sprite_renderers, mut locals, input, time): Self::SystemData,
    ) {
        // Move every ball according to its speed, and the time passed.
        for (player, sprite_render, local) in
            (&mut players, &mut sprite_renderers, &mut locals).join()
        {
            let movement = input.axis_value("horizontal");
            if let Some(mv_amount) = movement {
                player.velocity[0] = mv_amount * 20.0;
                if mv_amount < 0.0 {
                    if player.x_collision == Direction::LEFT {
                        player.velocity[0] = 0.0
                    };
                    if player.facing == Direction::RIGHT {
                        player.facing = Direction::LEFT;
                        local.set_scale(Vector3::new(
                            local.scale()[0] * -1.0,
                            local.scale()[1],
                            local.scale()[2],
                        ));
                    };
                } else if mv_amount > 0.0 {
                    if player.x_collision == Direction::RIGHT {
                        player.velocity[0] = 0.0
                    };
                    if player.facing == Direction::LEFT {
                        player.facing = Direction::RIGHT;
                        local.set_scale(Vector3::new(
                            local.scale()[0] * -1.0,
                            local.scale()[1],
                            local.scale()[2],
                        ));
                    };
                }
            }

            if player.velocity[0] != 0.0 {
                player.animation_counter -= time.delta_seconds();
            } else {
                sprite_render.sprite_number = 5
            }

            if player.animation_counter <= 0.0 {
                if sprite_render.sprite_number < 5 {
                    sprite_render.sprite_number = sprite_render.sprite_number + 1;
                } else {
                    sprite_render.sprite_number = 0
                }
                player.animation_counter = 0.07
            }
            local.prepend_translation_x(player.velocity[0] * time.delta_seconds() * 2.0);
        }
    }
}
