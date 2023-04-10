use bevy::prelude::{Component, Vec3};

// Common components
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn _zero() -> Self {
        Velocity { x: 0., y: 0. }
    }
    pub fn into_vec3(&self) -> Vec3 {
        Vec3 {x: self.x, y: self.y, z: 0.}
    }
    pub fn new(x: f32, y: f32) -> Self {
        Velocity { x, y }
    } 
}

#[derive(Component)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32,
}

impl Acceleration {
    pub fn zero() -> Self {
        Acceleration { x: 0., y: 0. }
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
}

#[derive(Component)]
pub struct Car;
