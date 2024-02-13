use bevy::prelude::*;
use crate::GameState;

#[derive(Component)]
pub struct Player {
    pub(crate) speed: f32,
    pub(crate) jump_index: u8,
}

const GRAVITY: f32 = 300.0;

pub fn character_movement(
    mut characters: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if game_state.is_running == false {
        return;
    }
    for (mut transform, mut player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();
        let gravity = GRAVITY * time.delta_seconds();
        if player.jump_index > 0 {
            transform.translation.y += movement_amount;
            player.jump_index += 1;
        } else if input.pressed(KeyCode::Space) {
            transform.translation.y += movement_amount;
            player.jump_index += 1;
        }
        if player.jump_index >= 40 {
            player.jump_index = 0;
        }
        if player.jump_index == 0 {
            transform.translation.y -= gravity;
        }
    }
}