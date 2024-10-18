use bevy::math::bounding::{Aabb2d, IntersectsVolume};

use crate::prelude::*;

const LASER_SPEED: f32 = 8.;
const CANNON_SPEED: f32 = 3.;
const BOMB_SPEED: f32 = 1.;
const UFO_SPEED: f32 = 1.;

#[derive(Clone, Copy, Debug)]
pub enum ControllerDirection {
    Right,
    Left,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct ControllerEvent {
    direction: ControllerDirection,
}

#[derive(Event, Debug)]
pub struct Fired;

pub fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut controller_event_writer: EventWriter<ControllerEvent>,
    mut fired_event_writer: EventWriter<Fired>,
) {
    if keys.pressed(KeyCode::ArrowLeft) {
        controller_event_writer.send(ControllerEvent {
            direction: ControllerDirection::Left,
        });
    }
    if keys.pressed(KeyCode::ArrowRight) {
        controller_event_writer.send(ControllerEvent {
            direction: ControllerDirection::Right,
        });
    }

    if keys.just_pressed(KeyCode::Space) {
        fired_event_writer.send(Fired {});
    }
}

pub fn move_cannon(
    mut cannon_transform_query: Query<&mut Transform, With<Cannon>>,
    mut controller_event_reader: EventReader<ControllerEvent>,
) {
    let mut cannon_transform = cannon_transform_query.single_mut();

    let mut direction: f32 = 0.;

    for controller_event in controller_event_reader.read() {
        match controller_event.direction {
            ControllerDirection::Left => direction -= 1.,
            ControllerDirection::Right => direction += 1.,
        }
    }

    let new_cannon_position = cannon_transform.translation.x + direction * CANNON_SPEED;

    cannon_transform.translation.x =
        new_cannon_position.clamp(LEFT_WALL + SPRITE_SIZE / 2., RIGHT_WALL - SPRITE_SIZE / 2.);
}

pub fn move_enemies(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    mut enemy_movement: ResMut<EnemyMovement>,
    mut enemy_advancement_event_writer: EventWriter<EnemyAdvancement>,
) {
    if enemy_movement.advance {
        enemy_query.iter_mut().for_each(|mut transform| {
            transform.translation.y -= SPRITE_SIZE;
        });
        enemy_movement.advance = false;
    } else {
        let mut advance = false;
        for mut transform in enemy_query.iter_mut() {
            let new_x = transform.translation.x + enemy_movement.speed * enemy_movement.direction;

            if new_x + SPRITE_SIZE > RIGHT_WALL || new_x - SPRITE_SIZE < LEFT_WALL {
                advance = true;
            }

            transform.translation.x = new_x;
        }

        if advance {
            enemy_advancement_event_writer.send_default();
            enemy_movement.reverse_direction();
            enemy_movement.advance = true;
        }
    }
}

pub fn fire_laser(
    mut commands: Commands,
    mut fired_event_reader: EventReader<Fired>,
    cannon_query: Query<&Transform, With<Cannon>>,
    laser_beam_query: Query<&LaserBeam>,
    sound: Res<FireLaserSound>,
) {
    // only one laser beam at the time
    if !laser_beam_query.is_empty() {
        return;
    }

    let cannon_transform = cannon_query.single();

    if !fired_event_reader.is_empty() {
        fired_event_reader.clear();
        commands.spawn(AudioBundle {
            source: sound.clone(),
            settings: PlaybackSettings::DESPAWN,
        });

        commands.spawn(LaserBeamBundle::new(
            cannon_transform.translation.x,
            cannon_transform.translation.y,
        ));
    }
}

pub fn move_laser_beam(
    mut commands: Commands,
    mut laser_beam_query: Query<(Entity, &mut Transform), With<LaserBeam>>,
) {
    for (entity, mut laser_beam_transform) in laser_beam_query.iter_mut() {
        laser_beam_transform.translation.y += 1. * LASER_SPEED;

        if laser_beam_transform.translation.y >= TOP_WALL {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn detect_laser_hit(
    mut commands: Commands,
    mut player: ResMut<Player>,
    laser_beam_query: Query<(Entity, &Transform, &Size), With<LaserBeam>>,
    hitable_query: Query<(Entity, &Transform, &Size, Option<&Hitpoints>), With<Hitable>>,
    mut hit_event_writer: EventWriter<HitEvent>,
) {
    if let Ok((laser_beam_entity, laser_beam_transform, laser_beam_size)) =
        laser_beam_query.get_single()
    {
        let laser_beam_bounding_box: Aabb2d = Aabb2d::new(
            laser_beam_transform.translation.truncate(),
            Vec2::new(laser_beam_size.width, laser_beam_size.height) / 2.,
        );

        for (entity, transform, hitable_size, maybe_hitpoints) in hitable_query.iter() {
            let mut points: i32 = 0;

            if let Some(hitpoints) = maybe_hitpoints {
                points = hitpoints.0;
            }

            let bounding_box: Aabb2d = Aabb2d::new(
                transform.translation.truncate(),
                Vec2::new(hitable_size.width, hitable_size.height) / 2.,
            );

            if bounding_box.intersects(&laser_beam_bounding_box) {
                hit_event_writer.send_default();
                player.add_to_score(points);

                commands.entity(laser_beam_entity).despawn();
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn play_enemy_hit_sound(
    mut commands: Commands,
    mut hit_event_reader: EventReader<HitEvent>,
    sound: Res<InvaderKilledSound>,
) {
    if !hit_event_reader.is_empty() {
        hit_event_reader.clear();
        commands.spawn(AudioBundle {
            source: sound.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

pub fn drop_bomb(
    mut commands: Commands,
    enemy_query: Query<(&Transform, &EnemyPosition), With<Enemy>>,
) {
    let positions = enemy_query
        .iter()
        .map(|(_transform, position)| position)
        .collect::<Vec<&EnemyPosition>>();

    fn is_edge(position: &EnemyPosition, positions: &Vec<&EnemyPosition>) -> bool {
        positions
            .iter()
            .filter(|grid_position| grid_position.x == position.x)
            .map(|column_position| column_position.y)
            .max()
            .unwrap()
            <= position.y
    }

    for (transform, position) in enemy_query.iter() {
        if !is_edge(position, &positions) {
            continue;
        }

        if random::<f32>() * 100. <= 10. {
            commands.spawn(BombBundle::new(
                transform.translation.x,
                transform.translation.y,
            ));
        }
    }
}

pub fn move_bomb(
    mut commands: Commands,
    mut bomb_query: Query<(Entity, &mut Transform), With<Bomb>>,
) {
    bomb_query
        .iter_mut()
        .for_each(|(entity, mut bomb_transform)| {
            bomb_transform.translation.y -= 1. * BOMB_SPEED;

            if bomb_transform.translation.y <= BOTTOM_WALL {
                commands.entity(entity).despawn();
            }
        });
}

pub fn detect_bomb_hit(
    mut commands: Commands,
    bomb_query: Query<(Entity, &Transform), With<Bomb>>,
    cannon_qery: Query<(&Transform, &Size), With<Cannon>>,
    mut player_hit_event_writer: EventWriter<PlayerHitEvent>,
    mut player: ResMut<Player>,
) {
    let (cannon_transform, size) = cannon_qery.single();

    let cannon_bounding_box = Aabb2d::new(
        cannon_transform.translation.truncate(),
        Vec2::new(size.width, size.height) / 2.,
    );

    for (bomb_entity, bomb_transform) in bomb_query.iter() {
        let bomb_bounding_box = Aabb2d::new(
            bomb_transform.translation.truncate(),
            bomb_transform.scale.truncate() / 2.,
        );

        if bomb_bounding_box.intersects(&cannon_bounding_box) {
            commands.entity(bomb_entity).despawn();

            player.kill();

            player_hit_event_writer.send_default();
        }
    }
}

pub fn update_lifes_ui(player: Res<Player>, mut lifes_ui_query: Query<&mut Text, With<LifesUI>>) {
    let mut text = lifes_ui_query.single_mut();
    text.sections[0].value = player.lifes_left();
}

pub fn update_score_ui(player: Res<Player>, mut score_ui_query: Query<&mut Text, With<ScoreUI>>) {
    let mut text = score_ui_query.single_mut();
    text.sections[0].value = player.score();
}

pub fn setup_ufo_timer(mut commands: Commands) {
    commands.spawn(UfoSpawnTimer(Timer::from_seconds(
        10.,
        TimerMode::Repeating,
    )));
}

pub fn move_ufo(
    mut commands: Commands,
    mut ufo_query: Query<(Entity, &mut Transform, &UfoDirection), With<Ufo>>,
) {
    if ufo_query.is_empty() {
        return;
    }

    let (entity, mut ufo_transform, direction) = ufo_query.single_mut();
    ufo_transform.translation.x += UFO_SPEED * direction.0;

    if ufo_transform.translation.x > RIGHT_WALL + SPRITE_SIZE
        || ufo_transform.translation.x < LEFT_WALL - SPRITE_SIZE
    {
        commands.entity(entity).despawn();
    }
}
