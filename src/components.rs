use crate::prelude::*;

#[derive(Resource)]
pub struct Player {
    lifes: i8,
    score: i32,
}

impl Player {
    pub fn new() -> Self {
        Self { lifes: 3, score: 0 }
    }

    pub fn kill(&mut self) {
        info!("Player died");
        self.lifes -= 1;

        if self.lifes == 0 {
            info!("GAME OVER!")
        }
    }

    pub fn lifes_left(&self) -> String {
        self.lifes.to_string()
    }

    pub fn add_to_score(&mut self, add: i32) {
        self.score += add;
    }

    pub fn score(&self) -> String {
        self.score.to_string()
    }
}

#[derive(Resource)]
pub struct EnemyMovement {
    pub direction: f32,
    pub speed: f32,
    pub advance: bool,
}

impl EnemyMovement {
    pub fn new() -> Self {
        Self {
            direction: 1.,
            speed: SPRITE_SIZE / 4.,
            advance: false,
        }
    }

    pub fn reverse_direction(&mut self) {
        self.direction *= -1.;
    }
}

#[derive(Event, Default)]
pub struct EnemyAdvancement;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Event, Default)]
pub struct PlayerHitEvent;

#[derive(Component, Clone, Copy, Debug)]
pub struct Cannon;

impl Cannon {
    pub fn size() -> Vec2 {
        Vec2::new(26., 16.)
    }
}

#[derive(Bundle)]
pub struct CannonBundle {
    marker: Cannon,
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
}

impl CannonBundle {
    pub fn new(texture: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Self {
        Self {
            marker: Cannon,
            sprite: SpriteBundle {
                texture,
                sprite: Sprite {
                    color: SPRITE_COLOR,
                    ..default()
                },
                transform: Transform {
                    translation: Vec2::new(0., BOTTOM_WALL + SPRITE_SIZE).extend(0.),
                    ..default()
                },
                ..default()
            },
            texture_atlas: TextureAtlas { layout, index: 0 },
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
            marker: LaserBeam,
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
pub struct Hitable;

#[derive(Component)]
pub struct Bomb {
    pub size: Vec2,
}

#[derive(Bundle)]
pub struct BombBundle {
    marker: Bomb,
    sprite: SpriteBundle,
    hitable: Hitable,
}

impl BombBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            marker: Bomb {
                size: Vec2::new(10., 30.),
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: SPRITE_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec2::new(10., 30.).extend(1.),
                    translation: Vec2::new(x, y).extend(0.),
                    ..default()
                },
                ..default()
            },
            hitable: Hitable,
        }
    }
}

#[derive(Component, Clone)]
pub struct Enemy {
    pub sprite_file_name: String,
    pub points: u8,
    pub size: Vec2,
}

impl Enemy {
    pub fn squid() -> Self {
        Enemy {
            sprite_file_name: "squid.png".into(),
            points: 30,
            size: Vec2::new(16., 16.),
        }
    }

    pub fn crab() -> Self {
        Enemy {
            sprite_file_name: "crab.png".into(),
            points: 20,
            size: Vec2::new(11., 16.),
        }
    }

    pub fn octopus() -> Self {
        Enemy {
            sprite_file_name: "octopus.png".into(),
            points: 10,
            size: Vec2::new(24., 16.),
        }
    }
}

#[derive(Component)]
pub struct EnemyPosition {
    pub x: u8,
    pub y: u8,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    marker: Enemy,
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
    pub position: EnemyPosition,
    hitable: Hitable,
}

impl EnemyBundle {
    pub fn new(
        enemy: Enemy,
        x: f32,
        y: f32,
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
    ) -> Self {
        Self {
            marker: enemy,
            sprite: SpriteBundle {
                texture,
                sprite: Sprite {
                    color: SPRITE_COLOR,
                    ..default()
                },
                transform: Transform {
                    translation: Vec2::new(x, y).extend(0.),
                    ..default()
                },
                ..default()
            },
            texture_atlas: TextureAtlas { layout, index: 0 },
            position: EnemyPosition { x: 0, y: 0 },
            hitable: Hitable,
        }
    }
}

#[derive(Component)]
pub struct LifesUI;

#[derive(Component)]
pub struct ScoreUI;
