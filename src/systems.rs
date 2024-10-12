use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum ControllerDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct ControllerEvent {
    direction: ControllerDirection,
}

pub fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut controller_event_writer: EventWriter<ControllerEvent>,
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
            _ => (),
        }
    }

    let new_cannon_position = cannon_transform.translation.x + direction;

    cannon_transform.translation.x =
        new_cannon_position.clamp(LEFT_WALL + SPRITE_SIZE / 2., RIGHT_WALL - SPRITE_SIZE / 2.);
}
