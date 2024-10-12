use crate::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_cannon(mut commands: Commands) {
    commands.spawn(CannonBundle::new());
}

pub fn spawn_enemies(mut commands: Commands) {
    // spawns a row of enemies
    for index in 0..11 {
        let offset = (SPRITE_SIZE + 1.) * index as f32 - SCREEN_WIDTH / 2. + SPRITE_SIZE;

        commands.spawn(EnemyBundle::new(offset, TOP_WALL - SPRITE_SIZE));
    }
}
