use bevy::prelude::{Component, Vec3};

// Common components
#[derive(Component)]
pub struct Velocity (pub f32);

impl Velocity {
    pub fn _zero() -> Self {
        Velocity (0.)
    }
    pub fn into_vec3(&self) -> Vec3 {
        Vec3 {x: 0., y: self.0, z: 0.}
    }
    pub fn new(val: f32) -> Self {
        Velocity(val)
    } 
}

#[derive(Component)]
pub struct Acceleration (pub f32);

impl Acceleration {
    pub fn _zero() -> Self {
        Acceleration (0.)
    }
}
//car components
#[derive(Component)]
pub struct Wheel;

#[derive(Component)]
pub struct RearWheel;

#[derive(Component)]
pub struct FrontWheel {
    pub turn_state: i8,
    pub turn_angle: f32,
}

#[derive(Component)]
pub struct Car;
