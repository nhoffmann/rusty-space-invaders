use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space Invader".into(),
                resolution: (224. * 2., 256. * 2.).into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
