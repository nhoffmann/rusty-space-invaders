mod components;
mod spawners;
mod systems;

mod prelude {
    pub use bevy::prelude::*;
    pub const SCREEN_WIDTH: f32 = 224. * 2.;
    pub const SCREEN_HEIGHT: f32 = 256. * 2.;
    pub const SPRITE_COLOR: Color = Color::srgb(1., 1., 1.);
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
}

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
        .add_systems(Startup, (spawn_camera, spawn_cannon).chain())
        .add_systems(Update, player_input)
        .add_systems(FixedUpdate, (move_cannon, fire_laser, move_laser_beam))
        .add_event::<ControllerEvent>()
        .add_event::<Fired>()
        .run();
}
