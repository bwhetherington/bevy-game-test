use bevy::prelude::*;

use crate::sprite::PIXEL_SCALE;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum PhysicsStep {
    Velocity,
    Pixel,
    Transform,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(velocity.system().label(PhysicsStep::Velocity))
            .add_system(
                update_pixel_position
                    .system()
                    .label(PhysicsStep::Pixel)
                    .after(PhysicsStep::Velocity),
            )
            .add_system(
                update_transform
                    .system()
                    .label(PhysicsStep::Transform)
                    .after(PhysicsStep::Pixel),
            );
    }
}

pub struct Position(pub Vec2);
pub struct Velocity(pub Vec2);

fn velocity(time: Res<Time>, mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in query.iter_mut() {
        pos.0 += vel.0 * time.delta_seconds();
    }
}

fn update_transform(mut query: Query<(&mut Transform, &PixelPosition)>) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation.x = pos.0.x;
        transform.translation.y = pos.0.y;
    }
}

pub struct PixelPosition(pub Vec2);

fn update_pixel_position(mut query: Query<(&mut PixelPosition, &Position)>) {
    for (mut pixel_pos, pos) in query.iter_mut() {
        pixel_pos.0.x = (pos.0.x / PIXEL_SCALE).trunc() * PIXEL_SCALE;
        pixel_pos.0.y = (pos.0.y / PIXEL_SCALE).trunc() * PIXEL_SCALE;
    }
}
