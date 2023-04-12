use bevy::prelude::*;
use car::CarPlugin;
use components::Car;

mod components;
mod car;

//car constants
const CAR_WIDTH: f32 = 50.; //width of body of car
const CAR_LENGTH: f32 = 100.; //length of body of car
const CAR_WHEEL_BASE: f32 = 80.; //distance between centres of wheels
const CAR_ACCELERATION: f32 = 0.2; //base acceleration
const CAR_AIR_RESISTANCE: f32 = 0.01; //for reducing acceleration based on speed (i.e car accelerates slower at high speeds)
const CAR_DECELERATION: f32 = 0.01; //drag/parasitic losses
const CAR_BRAKING_ACCELERATION: f32 = 0.2; //acceleration under braking
const CAR_MAX_SPEED: f32 = 50.; // hard max speed (car may reach actual max speed due to air resistance)

//wheel constants
const WHEEL_HEIGHT: f32 = 20.;
const WHEEL_WIDTH: f32 = 10.;
const WHEEL_TURN_ANGLE_MULT: f32 = 0.05;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Car Game!".to_string(),
                resolution: (600., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(CarPlugin)
        .add_startup_system(setup_system)
        .add_system(camera_follow_system)
        .run()
}

fn setup_system(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

fn camera_follow_system(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    car_query: Query<&mut Transform, (With<Car>, Without<Camera>)>
) {
    todo!("this does not work");
    // for mut camera in camera_query.iter_mut() {
    //     let car_tf = car_query.get_single().unwrap().translation;
    //     let car_tf2d = Vec2::new(car_tf.x, car_tf.y);
    //     let cam_2d = Vec2::new(camera.translation.x, camera.translation.y);
    //     if cam_2d.distance(car_tf2d) < 100. {
    //         continue;
    //     }
    //     let diff = car_tf2d - cam_2d;
    //     println!("{:?}", diff);
    //     camera.translation += Vec3::new((diff/30.).x, (diff/30.).y, 0.);
    // }
}

// fn move_system(
//     time: Res<Time>,
//     mut query: Query<(&mut Transform, &mut Acceleration, &mut Velocity), (With<Velocity>, With<Acceleration>)>
// ) {
//     for (mut tf, acc, mut vel) in query.iter_mut() {
//         vel.x += acc.x;
//         vel.y += acc.y;
//         tf.translation += Vec3::from((vel.x * time.delta_seconds(), vel.y * time.delta_seconds(), 0.));
//     }
// }