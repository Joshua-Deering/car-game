use core::panic;

use bevy::{prelude::{Plugin, Res, Input, Transform, With, Query, KeyCode, Quat, Commands, Color, Rect, Vec2, Vec3, Without}, sprite::{SpriteBundle, Sprite}};

use crate::{components::{FrontWheel, Wheel, Car, RearWheel, Velocity, Acceleration}, CAR_ACCELERATION, CAR_DECELERATION, CAR_WHEEL_BASE, WHEEL_HEIGHT, WHEEL_WIDTH, CAR_WIDTH, CAR_LENGTH, CAR_MAX_SPEED, CAR_BRAKING_ACCELERATION, WHEEL_TURN_ANGLE_MULT};

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(car_spawn_system)
        .add_startup_system(wheel_spawn_system)
        .add_system(car_move_system)
        .add_system(car_keyboard_control_system)
        .add_system(wheel_keyboard_control_system);
    }
}

fn car_move_system(
    mut rear_wheel_query: Query<&mut Transform, (With<Wheel>, Without<FrontWheel>)>,
    mut front_wheel_query: Query<(&mut Transform, &FrontWheel), (With<Wheel>, With<FrontWheel>)>,
    mut car_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), (With<Car>, Without<Wheel>)>
) {
    if let Ok((mut car, mut vel, acc)) = car_query.get_single_mut() {
        vel.0 += acc.0;
        vel.0 = vel.0.clamp(0., CAR_MAX_SPEED);

        let wheel = front_wheel_query.iter().next().unwrap().1;
        let mut turn_amt = 0.;
        if wheel.turn_state != 0 && !(vel.0 > -f32::EPSILON && vel.0 < f32::EPSILON) {
            let turn_radius = CAR_WHEEL_BASE / f32::sin(-wheel.turn_angle * wheel.turn_state as f32);
            turn_amt = vel.0 / turn_radius;
        }
        car.rotate(Quat::from_rotation_z(turn_amt));

        let rotation = car.rotation;
        let directional_vel = rotation * vel.into_vec3();
        car.translation += directional_vel;

        for mut wheel in rear_wheel_query.iter_mut() {
            wheel.translation += directional_vel;
            wheel.rotate_around(car.translation, Quat::from_rotation_z(turn_amt));
        }
        for (mut wheel, _) in front_wheel_query.iter_mut() {
            wheel.translation += directional_vel;
            wheel.rotate_around(car.translation, Quat::from_rotation_z(turn_amt));
        }
    }
}

fn car_keyboard_control_system(
    kb: Res<Input<KeyCode>>,
    mut car_query: Query<(&mut Acceleration, &mut Velocity), With<Car>>
) {
    let (mut acc, mut vel) = car_query.get_single_mut().unwrap();

    let mut kb_pressed = false;
    if kb.pressed(KeyCode::W) || kb.pressed(KeyCode::Up) {
        //acc.x += CAR_ACCELERATION;
        acc.0 = CAR_ACCELERATION;
        kb_pressed = true;
    }
    if kb.pressed(KeyCode::S) || kb.pressed(KeyCode::Down) {
        //acc.x -= CAR_ACCELERATION;
        acc.0 = -CAR_BRAKING_ACCELERATION;
        kb_pressed = true;
    }

    if !kb_pressed {
        if vel.0 < 0.2 {
            vel.0 = 0.;
            acc.0 = 0.;
            return;
        }
        acc.0 = -CAR_DECELERATION;
    }
}

fn car_spawn_system(
    mut commands: Commands
) {
    let mut car_entity = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::LIME_GREEN,
            rect: Some(Rect::new(0., 0., CAR_WIDTH, CAR_LENGTH)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 12.),
            ..Default::default()
        },
        ..Default::default()
    });
    car_entity.insert(Car);
    car_entity.insert(Velocity::new(1.));
    car_entity.insert(Acceleration::_zero());
}

fn wheel_spawn_system(
    mut commands: Commands
) {
    let mut spawn_wheel = |x_offset: f32, y_offset: f32, is_front_wheel: bool| {
        let mut wheel = commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                rect: Some(Rect{min: Vec2::new(0., 0.), max: Vec2::new(WHEEL_WIDTH, WHEEL_HEIGHT)}),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(x_offset, y_offset, 10.),
                ..Default::default()
            },
            ..Default::default()
        });
        wheel.insert(Wheel);
        if is_front_wheel {
            wheel.insert(FrontWheel{turn_state: 0, turn_angle: 0.});
        } else {
            wheel.insert(RearWheel);
        }
    };
    spawn_wheel(-CAR_WIDTH/2., CAR_WHEEL_BASE/2., true);
    spawn_wheel(CAR_WIDTH/2., CAR_WHEEL_BASE/2., true);
    spawn_wheel(-CAR_WIDTH/2., -CAR_WHEEL_BASE/2., false);
    spawn_wheel(CAR_WIDTH/2., -CAR_WHEEL_BASE/2., false);
}

fn wheel_keyboard_control_system(
    kb: Res<Input<KeyCode>>,
    mut wheel_query: Query<(&mut Transform, &mut FrontWheel), With<FrontWheel>>,
    car_query: Query<(&Transform, &Velocity), (With<Car>, Without<FrontWheel>)>,
) {
    let (car, vel) = car_query.get_single().unwrap();
    let turn_state: i8 = {
        if kb.pressed(KeyCode::A) ||  kb.pressed(KeyCode::Left) {
            -1
        } else if kb.pressed(KeyCode::D) ||  kb.pressed(KeyCode::Right) {
            1
        } else {
            0
        }
    };

    for (mut transform, mut wheel ) in wheel_query.iter_mut() {
        wheel.turn_angle = (0.785398 - (vel.0 * WHEEL_TURN_ANGLE_MULT)).max(0.01);
        if turn_state == 0 {
            transform.rotation = car.rotation;
            wheel.turn_state = 0;
            continue;
        }

        match turn_state {
            -1 => {
                if wheel.turn_state != turn_state {
                    transform.rotate(Quat::from_rotation_z(wheel.turn_angle));
                    wheel.turn_state = -1;
                }
                let diff = transform.rotation - car.rotation;
                if diff.z != wheel.turn_angle {
                    transform.rotation = car.rotation;
                    transform.rotate(Quat::from_rotation_z(wheel.turn_angle));
                }
            },
            1 => {
                if wheel.turn_state != turn_state {
                    transform.rotate(Quat::from_rotation_z(-wheel.turn_angle));
                    wheel.turn_state = 1;
                }
                let diff = transform.rotation - car.rotation;
                if diff.z != -wheel.turn_angle {
                    transform.rotation = car.rotation;
                    transform.rotate(Quat::from_rotation_z(-wheel.turn_angle));
                }
            },
            0 => {
                transform.rotation = car.rotation;
            },
            _ => {
                panic!("invalid value for turn_state ({})", turn_state);
            }
        }
    }
}