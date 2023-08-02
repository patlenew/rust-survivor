use bevy::prelude::*;
use rand::Rng;
use std::{env, f32::consts::PI};

mod main_loop;
pub use crate::main_loop::external;

#[derive(Component)]
pub struct EntityName {
    pub name: String
}

#[derive(Component)]
pub struct Player {
}
    
#[derive(Component)]
pub struct Enemy {
}

#[derive(Component)]
pub struct SpellBook {
    // Add spells
}

#[derive(Component)]
pub struct StatSheet {
    pub health: u32,
    pub damage: u32,
    pub move_speed: f32,
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup_test)
    .add_systems(Update, main_loop::update)
    .add_systems(Update, main_loop::update_player_pos)
    .add_systems(Update, main_loop::update_cam_pos)
    .run();
}

fn setup_test(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let mut i = 0;
    let radius: f32 = 200.0;

    while i < 10 {

        let angle: f32 = rand::thread_rng().gen_range(0.0..360.0);
        let radian_angle = angle * PI/180.0;
        
        let x = radian_angle.cos() * radius;
        let y = radian_angle.sin() * radius;

        commands.spawn((
            SpriteBundle {
            texture: asset_server.load("sprites/enemy_test.png"),
            transform:  Transform::from_xyz(x, y, 0.0),
            ..default()
        }, 
        EntityName {
            name: "Enemy Of Not Cool".to_string(),
        },
        StatSheet {
            health: 5,
            damage: 1,
            move_speed: 5.0,
        }
        ));

        i += 1;
    }

    // Spawning player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/player.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player {

        },
        EntityName {
            name: "Player Of Cool".to_string(),
        },
        StatSheet {
            health: 10,
            damage: 2,
            move_speed: 100.0,
        }
    ));

}
