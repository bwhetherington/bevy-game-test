use bevy::prelude::*;

use crate::{
    movement::Movement,
    physics::{PixelPosition, Position, Velocity},
    sprite::{Animation, Animations, PIXEL_SCALE},
};

pub struct Character;

pub fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Texture> = asset_server.load("spritesheets/male.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.1, 32.1), 3, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let transform = Transform::from_scale(Vec3::splat(PIXEL_SCALE));

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::default(),
            transform,
            ..Default::default()
        })
        .insert(Position(Vec2::default()))
        .insert(PixelPosition(Vec2::default()))
        .insert(Velocity(Vec2::default()))
        .insert(Character)
        .insert(Movement { last_angle: 0 })
        .insert(Animations {
            current_animation: 0,
            animations: vec![
                Animation {
                    current_frame: 0,
                    frames: vec![1],
                },
                Animation {
                    current_frame: 0,
                    frames: vec![0, 2],
                },
                Animation {
                    current_frame: 0,
                    frames: vec![10],
                },
                Animation {
                    current_frame: 0,
                    frames: vec![9, 11],
                },
                Animation {
                    current_frame: 0,
                    frames: vec![7],
                },
                Animation {
                    current_frame: 0,
                    frames: vec![6, 8],
                },
                Animation {
                    current_frame: 0,
                    frames: vec![4],
                },
                Animation {
                    current_frame: 0,
                    frames: vec![3, 5],
                },
            ],
        });
}
