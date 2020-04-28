use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

use crate::components::{Direction, Door, Player};

#[derive(SystemDesc)]
pub struct DoorCollisions;

impl<'s> System<'s> for DoorCollisions {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Door>,
    );

    fn run(
        &mut self,
        (mut players, mut transforms, mut sprite_renderers, doors): Self::SystemData,
    ) {
        for (door, door_transform) in (&doors, &transforms).join() {
            let door_left_collider = (
                door_transform.translation().x - (door.width * 0.5),
                door_transform.translation().y - (door.height * 0.5),
            );

            for (player, player_transform) in (&mut players, &transforms).join() {
                let player_collider = (
                    player_transform.translation().x - (player.width * 0.5),
                    player_transform.translation().y - (player.height * 0.5),
                );

                if (player_collider.0 < door_left_collider.0 + door.width)
                    && (player_collider.0 + player.width > door_left_collider.0)
                    && (player_collider.1 < door_left_collider.1 + door.height)
                    && (player_collider.1 + player.height > door_left_collider.1)
                {
                    player.x_collision = player.facing;
                } else {
                    player.x_collision = Direction::NONE;
                };
            }
        }
    }
}
