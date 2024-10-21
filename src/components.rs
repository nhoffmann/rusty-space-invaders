use rand::Rng;

use crate::prelude::*;

#[derive(Resource)]
pub struct Player {
    pub lifes: i8,
    pub score: i32,
}

impl Player {
    pub fn new() -> Self {
        Self { lifes: 3, score: 0 }
    }

    pub fn kill(&mut self) {
        info!("Player died");
        self.lifes -= 1;
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
pub struct HitEvent;

#[derive(Event, Default)]
pub struct PlayerHitEvent;

#[derive(Component, Clone, Copy, Debug)]
pub struct Cannon;

#[derive(Bundle)]
pub struct CannonBundle {
    marker: Cannon,
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
    size: Size,
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
            size: Size {
                width: 26.,
                height: 16.,
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
    size: Size,
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
                    scale: Vec2::new(3., 10.).extend(1.),
                    translation: Vec2::new(x, y + SPRITE_SIZE / 2.).extend(0.),
                    ..default()
                },
                ..default()
            },
            size: Size {
                width: 1.,
                height: 10.,
            },
        }
    }
}

#[derive(Component)]
pub struct Hitable;

#[derive(Component)]
pub struct Bomb;

#[derive(Bundle)]
pub struct BombBundle {
    marker: Bomb,
    sprite: SpriteBundle,
    hitable: Hitable,
    size: Size,
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl BombBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            marker: Bomb,
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
            size: Size {
                width: 10.,
                height: 30.,
            },
        }
    }
}

#[derive(Component, Clone)]
pub struct Enemy {
    pub sprite_file_name: String,
    points: i32,
    width: f32,
    height: f32,
}

impl Enemy {
    pub fn squid() -> Self {
        Enemy {
            sprite_file_name: "squid.png".into(),
            points: 30,
            width: 16.,
            height: 16.,
        }
    }

    pub fn crab() -> Self {
        Enemy {
            sprite_file_name: "crab.png".into(),
            points: 20,
            width: 11.,
            height: 16.,
        }
    }

    pub fn octopus() -> Self {
        Enemy {
            sprite_file_name: "octopus.png".into(),
            points: 10,
            width: 24.,
            height: 16.,
        }
    }
}

#[derive(Component)]
pub struct EnemyPosition {
    pub x: u8,
    pub y: u8,
}

#[derive(Component)]
pub struct Hitpoints(pub i32);

#[derive(Bundle)]
pub struct EnemyBundle {
    marker: Enemy,
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
    pub position: EnemyPosition,
    hitable: Hitable,
    size: Size,
    hitpoints: Hitpoints,
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
            marker: enemy.clone(),
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
            size: Size {
                width: enemy.width,
                height: enemy.height,
            },
            hitpoints: Hitpoints(enemy.points),
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct UfoSpawnTimer(pub Timer);

#[derive(Component)]
pub struct Ufo;

#[derive(Component)]
pub struct UfoDirection(pub f32);

#[derive(Bundle)]
pub struct UfoBundle {
    marker: Ufo,
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
    hitable: Hitable,
    direction: UfoDirection,
    size: Size,
    hitpoints: Hitpoints,
}

impl UfoBundle {
    pub fn new(
        x: f32,
        y: f32,
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        direction: f32,
    ) -> Self {
        let possible_hitpoints = [50, 100, 150, 200, 300];
        let hp_index = thread_rng().gen_range(0..5);
        let hitpoints = Hitpoints(possible_hitpoints[hp_index]);

        Self {
            marker: Ufo,
            sprite: SpriteBundle {
                texture,
                transform: Transform::from_translation(Vec2::new(x, y).extend(0.)),
                ..default()
            },
            texture_atlas: TextureAtlas { layout, index: 0 },
            hitable: Hitable,
            direction: UfoDirection(direction),
            size: Size {
                width: 32.,
                height: 14.,
            },
            hitpoints,
        }
    }
}

#[derive(Component)]
pub struct LifesUI;

#[derive(Component)]
pub struct ScoreUI;

#[derive(Resource, Debug)]
pub struct Difficulty(pub i32);

impl Default for Difficulty {
    fn default() -> Self {
        Self(100)
    }
}

#[derive(Component, Debug)]
pub struct Menu;

#[derive(Component, Debug)]
pub struct GameOverSign;

#[derive(Resource, Debug)]
pub struct Level(pub f32);
