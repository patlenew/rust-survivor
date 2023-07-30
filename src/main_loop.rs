use bevy::prelude::*;

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