use bevy::prelude::*;
use car::CarPlugin;

mod components;
mod car;

//resources

const CAR_ACCELERATION: f32 = 0.1;
const CAR_DECELERATION: f32 = 0.1;

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
        .run()
}

fn setup_system(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
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