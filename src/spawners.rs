use crate::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_cannon(mut commands: Commands) {
    commands.spawn(CannonBundle::new());
}

pub fn spawn_enemies(mut commands: Commands) {
    let mut y = TOP_WALL - SPRITE_SIZE;

    // spawns a row of enemies
    for _row in 0..5 {
        for col in 0..11 {
            let offset = (SPRITE_SIZE + 1.) * col as f32 - SCREEN_WIDTH / 2. + SPRITE_SIZE;

            commands.spawn(EnemyBundle::new(offset, y));
        }

        // switch to next line
        y -= SPRITE_SIZE + 1.;
    }
}
