use crate::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup(mut commands: Commands) {
    let difficulty = Difficulty::default();
    commands.insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(
        (difficulty.0 * 10) as u64,
    )));
    commands.insert_resource(difficulty);

    commands.insert_resource(EnemyMovement::new());
    commands.insert_resource(Player::new());
}

pub fn spawn_cannon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = asset_server.load("cannon.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(CannonBundle::new(texture, texture_atlas_layout));
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

pub fn spawn_ufo(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_ufo_timer_query: Query<&mut UfoSpawnTimer>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    enemy_movement: Res<EnemyMovement>,
) {
    for mut timer in &mut spawn_ufo_timer_query {
        if !timer.tick(time.delta()).just_finished() {
            return;
        }
    }
    let texture: Handle<Image> = asset_server.load("ufo.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 1, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let start_x: f32 = match enemy_movement.direction > 0. {
        true => LEFT_WALL + SPRITE_SIZE / 2.,
        false => RIGHT_WALL - SPRITE_SIZE / 2.,
    };

    commands.spawn(UfoBundle::new(
        start_x,
        TOP_WALL - SPRITE_SIZE / 2.,
        texture.clone(),
        texture_atlas_layout.clone(),
        enemy_movement.direction,
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

pub fn despawn_game(
    mut commands: Commands,
    enemies_query: Query<Entity, With<Enemy>>,
    score_ui_query: Query<Entity, With<ScoreUI>>,
    lifes_ui_query: Query<Entity, With<LifesUI>>,
) {
    enemies_query
        .iter()
        .for_each(|enemy| commands.entity(enemy).despawn_recursive());
    score_ui_query
        .iter()
        .for_each(|enemy| commands.entity(enemy).despawn_recursive());
    lifes_ui_query
        .iter()
        .for_each(|enemy| commands.entity(enemy).despawn_recursive());
}

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum ButtonAction {
    StartGame,
}

pub fn spawn_menu(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                ..default()
                            },
                            ButtonAction::StartGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Start Game",
                                button_text_style.clone(),
                            ));
                        });
                });
        });
}

pub fn despawn_menu(mut commands: Commands, menu_query: Query<Entity, With<Menu>>) {
    if let Ok(menu) = menu_query.get_single() {
        commands.entity(menu).despawn_recursive();
    }
}
