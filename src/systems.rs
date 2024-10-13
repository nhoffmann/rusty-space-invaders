use bevy::math::bounding::{Aabb2d, IntersectsVolume};

use crate::prelude::*;

const LASER_SPEED: f32 = 8.;
const CANNON_SPEED: f32 = 3.;
const BOMB_SPEED: f32 = 1.;

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

pub fn move_enemies_horizontal(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    mut enemy_movement: ResMut<EnemyMovement>,
    mut enemy_advancement_event_writer: EventWriter<EnemyAdvancement>,
) {
    enemy_query.iter_mut().for_each(|mut transform| {
        transform.translation.x += enemy_movement.speed * enemy_movement.direction;

        if transform.translation.x + SPRITE_SIZE / 2. >= RIGHT_WALL {
            enemy_movement.level_up();
            enemy_advancement_event_writer.send_default();
        }

        if transform.translation.x - SPRITE_SIZE / 2. < LEFT_WALL {
            enemy_movement.level_up();
            enemy_advancement_event_writer.send_default();
        }
    });
}

pub fn move_enemies_vertical(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    mut enemy_advancement_event_reader: EventReader<EnemyAdvancement>,
) {
    for _ in enemy_advancement_event_reader.read() {
        enemy_query.iter_mut().for_each(|mut transform| {
            transform.translation.y -= SPRITE_SIZE;
        });
    }
}

pub fn fire_laser(
    mut commands: Commands,
    mut fired_event_reader: EventReader<Fired>,
    cannon_query: Query<&Transform, With<Cannon>>,
    laser_beam_query: Query<&LaserBeam>,
) {
    // only one laser beam at the time
    if laser_beam_query.iter().collect::<Vec<&LaserBeam>>().len() > 0 {
        return;
    }

    let cannon_transform = cannon_query.single();

    for _ in fired_event_reader.read() {
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
    laser_beam_query: Query<(Entity, &Transform), With<LaserBeam>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    bomb_query: Query<(Entity, &Transform), With<Bomb>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    if let Ok((laser_beam_entity, laser_beam_transform)) = laser_beam_query.get_single() {
        let laser_beam_bounding_box: Aabb2d = Aabb2d::new(
            laser_beam_transform.translation.truncate(),
            laser_beam_transform.scale.truncate() / 2.,
        );

        for (entity, transform) in enemy_query.iter().chain(bomb_query.iter()) {
            let bounding_box: Aabb2d = Aabb2d::new(
                transform.translation.truncate(),
                transform.scale.truncate() / 2.,
            );

            if bounding_box.intersects(&laser_beam_bounding_box) {
                collision_event_writer.send_default();

                commands.entity(laser_beam_entity).despawn();
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn drop_bomb(mut commands: Commands, enemy_query: Query<&Transform, With<Enemy>>) {
    for transform in enemy_query.iter() {
        if random::<f32>() * 1000. <= 1. {
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
    cannon_qery: Query<&Transform, With<Cannon>>,
    mut player_hit_event_writer: EventWriter<PlayerHitEvent>,
    mut player: ResMut<Player>,
) {
    let cannon_transform = cannon_qery.single();

    let cannon_bounding_box = Aabb2d::new(
        cannon_transform.translation.truncate(),
        cannon_transform.scale.truncate() / 2.,
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
