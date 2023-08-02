use bevy::prelude::*;

use crate::{Player, StatSheet};

pub mod external {

}

#[derive(Component)]
pub struct EnemySpawnCountdown {
    pub countBeforeSpawnWave: f32,
}

pub fn update() {
    // println!("yeaah");
}

pub fn update_enemy_pos() {
    
}

pub fn update_player_pos(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    player_stats_query: Query<&StatSheet, With<Player>>,
    time: Res<Time>
) {

    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() != 0.0 {
            direction = direction.normalize();
        }

        let player_stats: &StatSheet = player_stats_query.single();

        transform.translation += direction * player_stats.move_speed * time.delta_seconds();


    }
}

pub fn update_cam_pos(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>
) {

    if let Ok(mut camera_transform) = camera_query.get_single_mut() {

        let transform = player_query.single();

        let cam_pos = Vec3::new(
            transform.translation.x,
            transform.translation.y,
            camera_transform.translation.z, // keep the old Z translation
        );

        camera_transform.translation = cam_pos;
    }
}