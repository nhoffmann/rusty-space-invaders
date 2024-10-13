use crate::prelude::*;

#[derive(Resource)]
pub struct EnemyMovement {
    pub direction: f32,
    pub speed: f32,
}

impl EnemyMovement {
    pub fn new() -> Self {
        Self {
            direction: 1.,
            speed: 0.4,
        }
    }

    pub fn level_up(&mut self) {
        self.reverse_direction();
        self.increase_speed();
    }

    fn reverse_direction(&mut self) {
        self.direction *= -1.;
    }

    fn increase_speed(&mut self) {
        self.speed += 0.02;
    }
}

#[derive(Event, Default)]
pub struct EnemyAdvancement;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Component, Clone, Copy, Debug)]
pub struct Cannon;

#[derive(Bundle)]
pub struct CannonBundle {
    marker: Cannon,
    sprite: SpriteBundle,
}

impl CannonBundle {
    pub fn new() -> Self {
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

#[derive(Bundle)]
pub struct LaserBeamBundle {
    marker: LaserBeam,
    sprite: SpriteBundle,
}

impl LaserBeamBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            marker: LaserBeam {},
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: SPRITE_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec2::new(1., 10.).extend(1.),
                    translation: Vec2::new(x, y + SPRITE_SIZE / 2.).extend(0.),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    marker: Enemy,
    sprite: SpriteBundle,
}

impl EnemyBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            marker: Enemy {},
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: SPRITE_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec2::new(SPRITE_SIZE, SPRITE_SIZE).extend(1.),
                    translation: Vec2::new(x, y).extend(0.),
                    ..default()
                },
                ..default()
            },
        }
    }
}
