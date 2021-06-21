use bevy::prelude::*;

pub trait Angle {
    fn angle(&self) -> f32;
}

impl Angle for Vec2 {
    fn angle(&self) -> f32 {
        println!("atan2 {}", self.y.atan2(self.y));
        (self.y.atan2(self.x) + std::f32::consts::TAU * 2.0) % std::f32::consts::TAU
    }
}
