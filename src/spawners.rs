use crate::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_cannon(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: SPRITE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(SPRITE_SIZE, SPRITE_SIZE, 0.),
                translation: Vec2::new(0., BOTTOM_WALL + SPRITE_SIZE).extend(0.),
                ..default()
            },
            ..default()
        },
        Cannon {},
    ));
}
