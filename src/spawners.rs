use bevy::render::texture;

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

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut y = TOP_WALL - SPRITE_SIZE;

    // spawns a row of enemies
    for row in 0..5 {
        let enemy = match row {
            0 => Enemy::squid(),
            1 => Enemy::crab(),
            2 => Enemy::crab(),
            _ => Enemy::octopus(),
        };

        let texture: Handle<Image> = asset_server.load(enemy.sprite_file_name.clone());
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        for col in 0..11 {
            let offset = (SPRITE_SIZE + 1.) * col as f32 - SCREEN_WIDTH / 2. + SPRITE_SIZE;
            let mut enemy_bundle = EnemyBundle::new(
                enemy.clone(),
                offset,
                y,
                texture.clone(),
                texture_atlas_layout.clone(),
            );
            enemy_bundle.position = EnemyPosition { x: col, y: row };

            commands.spawn(enemy_bundle);
        }

        // switch to next line
        y -= SPRITE_SIZE + 1.;
    }
}
