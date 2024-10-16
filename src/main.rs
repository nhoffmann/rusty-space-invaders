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
        // TODO should be a timer, see below
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
            ),
        )
        // TODO: do this in a timer, so we can control the interval, i.e. shorten it when time progresses
        .add_systems(
            FixedUpdate,
            (move_enemies, drop_bomb, play_invader_sound, spawn_ufo),
        )
        .add_event::<ControllerEvent>()
        .add_event::<Fired>()
        .add_event::<HitEvent>()
        .add_event::<EnemyAdvancement>()
        .add_event::<PlayerHitEvent>()
        .run();
}
