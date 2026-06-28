use bevy::prelude::*;
use crate::scripting::WindowId;
use crate::walkmesh;

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct Movement
    {
    pub speed_x: f32,
    pub speed_y: f32,
    }

    pub fn move_player(mut query: Query<(&mut Transform, &Movement, &mut Sprite)>, window_query: Query<&WindowId>, 
    input: Res<ButtonInput<KeyCode>>, time: Res<Time>, walk_area: Res<walkmesh::WalkableArea>, 
    player_images: Res<PlayerImages>) {
        if !window_query.is_empty() {
            return;
        }
        for (mut transform, movement, mut sprite) in query.iter_mut() {
            let mut direction = Vec2::ZERO;
            if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
                direction.x -= 1.0;
            }
            if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
                direction.x += 1.0;
            }
            if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
                direction.y += 1.0;
            }
            if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
                direction.y -= 1.0;
            }

            if direction != Vec2::ZERO {
                direction = direction.normalize();
                transform.translation.x += direction.x * movement.speed_x  * time.delta_secs();
                transform.translation.y += direction.y * movement.speed_y  * time.delta_secs();
            }

            if direction.y < 0.0 {
                sprite.image = player_images.player_down.clone();
            } else if direction.y > 0.0 {
                sprite.image = player_images.player_up.clone();
            }

            if direction.x < 0.0 {
                sprite.image = player_images.player_left.clone();
            } else if direction.x > 0.0 {
                sprite.image = player_images.player_right.clone();
            }

            println!("Player position: ({}, {})", transform.translation.x, transform.translation.y);

            transform.translation = walk_area.clamp_position(transform.translation);
        }
    }

#[derive(Resource)]
pub struct PlayerImages {
    pub player_left: Handle<Image>,
    pub player_right: Handle<Image>,
    pub player_up: Handle<Image>,
    pub player_down: Handle<Image>,
}