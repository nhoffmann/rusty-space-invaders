use crate::prelude::*;

const LASER_SPEED: f32 = 6.;

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

    let new_cannon_position = cannon_transform.translation.x + direction;

    cannon_transform.translation.x =
        new_cannon_position.clamp(LEFT_WALL + SPRITE_SIZE / 2., RIGHT_WALL - SPRITE_SIZE / 2.);
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
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: SPRITE_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec2::new(1., 10.).extend(0.),
                    translation: Vec2::new(
                        cannon_transform.translation.x,
                        cannon_transform.translation.y + SPRITE_SIZE / 2.,
                    )
                    .extend(0.),
                    ..default()
                },
                ..default()
            },
            LaserBeam {},
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
