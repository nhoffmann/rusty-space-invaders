mod components;
mod sounds;
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
    pub use crate::sounds::*;
    pub use crate::spawners::*;
    pub use crate::systems::*;
    pub use bevy::prelude::*;
    pub use rand::prelude::random;
    pub use rand::prelude::thread_rng;
    pub use std::time::Duration;
}

use prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
    LevelComplete,
    GameOver,
}

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
        .init_state::<GameState>()
        // Systems
        .add_systems(Startup, ((load_sounds, spawn_camera),))
        .add_systems(OnEnter(GameState::Menu), spawn_menu)
        .add_systems(OnExit(GameState::Menu), (despawn_menu, setup_player))
        .add_systems(
            OnEnter(GameState::LevelComplete),
            (despawn_game, start_next_level),
        )
        .add_systems(OnEnter(GameState::GameOver), spawn_menu)
        .add_systems(
            OnExit(GameState::GameOver),
            (despawn_menu, despawn_game, setup_player),
        )
        .add_systems(
            OnEnter(GameState::Playing),
            (
                reset,
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
                move_cannon.after(player_input),
                fire_laser.after(player_input),
                move_laser_beam,
                move_ufo,
                move_bomb,
                detect_laser_hit,
                detect_bomb_hit,
                play_enemy_hit_sound.after(detect_laser_hit),
                update_score_ui.after(detect_laser_hit),
                update_lifes_ui.after(detect_bomb_hit),
                check_game_over,
                check_level_complete,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (handle_menu_buttons).run_if(in_state(GameState::Menu)),
        )
        .add_systems(
            Update,
            (handle_menu_buttons).run_if(in_state(GameState::GameOver)),
        )
        .add_systems(
            FixedUpdate,
            (
                move_enemies,
                drop_bomb,
                play_invader_sound,
                spawn_ufo,
                increase_difficulty.after(move_enemies),
            )
                .run_if(in_state(GameState::Playing)),
        )
        // Events
        .add_event::<ControllerEvent>()
        .add_event::<Fired>()
        .add_event::<HitEvent>()
        .add_event::<EnemyAdvancement>()
        .add_event::<PlayerHitEvent>()
        .run();
}
