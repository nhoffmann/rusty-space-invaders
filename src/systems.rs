use bevy::math::bounding::{Aabb2d, IntersectsVolume};

use crate::prelude::*;

const LASER_SPEED: f32 = 8.;
const CANNON_SPEED: f32 = 3.;

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
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    if let Ok((laser_beam_entity, laser_beam_transform)) = laser_beam_query.get_single() {
        let laser_beam_bounding_box: Aabb2d = Aabb2d::new(
            laser_beam_transform.translation.truncate(),
            laser_beam_transform.scale.truncate() / 2.,
        );

        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let enemy_bounding_box: Aabb2d = Aabb2d::new(
                enemy_transform.translation.truncate(),
                enemy_transform.scale.truncate() / 2.,
            );

            if enemy_bounding_box.intersects(&laser_beam_bounding_box) {
                collision_event_writer.send_default();

                commands.entity(laser_beam_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
