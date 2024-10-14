mod components;
mod spawners;
mod systems;

mod prelude {
    pub const SCREEN_WIDTH: f32 = 224. * 2.;
    pub const SCREEN_HEIGHT: f32 = 256. * 2.;
    pub const SPRITE_COLOR: Color = Color::srgb(1., 1., 1.);
    pub const TEXT_COLOR: Color = Color::srgb(1., 1., 1.);
    pub const TEXT_SIZE: f32 = 32.;
    pub const TOP_MENU_HEIGHT: f32 = 50.;
    pub const BOTTOM_MENU_HEIGHT: f32 = 30.;
    pub const TOP_WALL: f32 = (SCREEN_HEIGHT / 2.) - TOP_MENU_HEIGHT;
    pub const RIGHT_WALL: f32 = SCREEN_WIDTH / 2.;
    pub const BOTTOM_WALL: f32 = SCREEN_HEIGHT / -2. + BOTTOM_MENU_HEIGHT;
    pub const LEFT_WALL: f32 = SCREEN_WIDTH / -2.;
    pub const SPRITE_SIZE: f32 = 32.;

    pub use crate::components::*;
    pub use crate::spawners::*;
    pub use crate::systems::*;
    pub use bevy::prelude::*;
    pub use rand::prelude::random;
}

use std::time::Duration;

use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space Invader".into(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
        .insert_resource(EnemyMovement::new())
        .insert_resource(Player::new())
        .add_systems(
            Startup,
            (
                load_sounds,
                spawn_camera,
                spawn_cannon,
                spawn_enemies,
                spawn_lifes_ui,
                spawn_score_ui,
                setup_ufo_timer,
            ),
        )
        .add_systems(
            Update,
            (
                player_input,
                update_lifes_ui,
                update_score_ui,
                move_cannon,
                fire_laser,
                move_laser_beam,
                detect_laser_hit,
                move_bomb,
                detect_bomb_hit,
                play_enemy_hit_sound,
                move_ufo,
            ),
        )
        .add_systems(FixedUpdate, (move_enemies, drop_bomb, spawn_ufo))
        .add_event::<ControllerEvent>()
        .add_event::<Fired>()
        .add_event::<HitEvent>()
        .add_event::<EnemyAdvancement>()
        .add_event::<PlayerHitEvent>()
        .run();
}
