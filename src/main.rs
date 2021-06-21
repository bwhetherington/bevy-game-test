use bevy::prelude::*;

mod character;
mod math;
mod movement;
mod physics;
mod sprite;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(sprite::AnimationPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(character::spawn_character.system())
        .run();
}
