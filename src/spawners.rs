use crate::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_cannon(mut commands: Commands) {
    commands.spawn(CannonBundle::new());
}

pub fn spawn_lifes_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font_size: TEXT_SIZE,
            color: TEXT_COLOR,
            ..default()
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(SCREEN_HEIGHT - BOTTOM_MENU_HEIGHT),
            left: Val::Px(0.),
            ..default()
        }),
        LifesUI,
    ));
}

pub fn spawn_score_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font_size: TEXT_SIZE,
            color: TEXT_COLOR,
            ..default()
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            left: Val::Px(0.),
            ..default()
        }),
        ScoreUI,
    ));
}

pub fn spawn_enemies(mut commands: Commands) {
    let mut y = TOP_WALL - SPRITE_SIZE;

    // spawns a row of enemies
    for _row in 0..5 {
        for col in 0..11 {
            let offset = (SPRITE_SIZE + 1.) * col as f32 - SCREEN_WIDTH / 2. + SPRITE_SIZE;

            commands.spawn(EnemyBundle::new(offset, y));
        }

        // switch to next line
        y -= SPRITE_SIZE + 1.;
    }
}
