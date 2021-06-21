use bevy::prelude::*;

use crate::{math::Angle, physics::Velocity};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(movement.system());
    }
}

pub struct Movement {
    pub last_angle: u8,
}

fn get_angle_index(vec: Vec2) -> u8 {
    let angle = vec.angle();
    println!("angle: {}", angle);
    let angle = vec.angle() / std::f32::consts::TAU;
    let angle = (angle * 4.0).round();
    angle as u8
}

fn movement(input: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &mut Movement)>) {
    let mut angle = Vec2::new(0.0, 0.0);

    if input.pressed(KeyCode::A) {
        angle.x -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        angle.x += 1.0;
    }
    if input.pressed(KeyCode::W) {
        angle.y += 1.0;
    }
    if input.pressed(KeyCode::S) {
        angle.y -= 1.0;
    }

    if let Some(norm) = angle.try_normalize() {
        angle = norm;
    }

    for (mut velocity, mut movement) in query.iter_mut() {
        velocity.0 = angle * 100.0;
        if (velocity.0 as Vec2).distance_squared(Vec2::ZERO) > 0.0 {
            let angle = get_angle_index(velocity.0);
            movement.last_angle = angle;
        }
    }
}
