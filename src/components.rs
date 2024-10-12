use crate::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Cannon;

#[derive(Bundle)]
pub struct CannonBundle {
    marker: Cannon,
    sprite: SpriteBundle,
}

impl Default for CannonBundle {
    fn default() -> Self {
        Self {
            marker: Cannon {},
            sprite: SpriteBundle {
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
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct LaserBeam;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    marker: Enemy,
    sprite: SpriteBundle,
}

impl EnemyBundle {
    pub fn transform(&mut self, x: f32, y: f32) {
        self.sprite.transform.translation = Vec2::new(x, y).extend(0.);
    }
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            marker: Enemy {},
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: SPRITE_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec2::new(SPRITE_SIZE, SPRITE_SIZE).extend(0.),
                    ..default()
                },
                ..default()
            },
        }
    }
}
