use bevy::{core::FixedTimestep, prelude::*};

use crate::{movement::Movement, physics::Velocity};

pub const PIXEL_SCALE: f32 = 4.0;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .with_system(animate.system())
                .with_run_criteria(FixedTimestep::step(1.0 / 4.0)),
        )
        .add_system(update_sprite_atlas.system())
        .add_system(animate_movement.system());
    }
}

pub struct Animation {
    pub frames: Vec<usize>,
    pub current_frame: usize,
}

pub struct Animations {
    pub animations: Vec<Animation>,
    pub current_animation: usize,
}

impl Animations {
    pub fn set_animation(&mut self, index: usize) {
        if index != self.current_animation {
            self.current_animation = index;
            if let Some(mut anim) = self.get_animation_mut() {
                anim.current_frame = 0;
            }
        }
    }

    pub fn get_animation(&self) -> Option<&Animation> {
        self.animations.get(self.current_animation)
    }

    pub fn get_animation_mut(&mut self) -> Option<&mut Animation> {
        self.animations.get_mut(self.current_animation)
    }
}

fn update_sprite_atlas(mut query: Query<(&mut TextureAtlasSprite, &Animations)>) {
    for (mut sprite, animations) in query.iter_mut() {
        let current_anim_index = animations.current_animation;
        if let Some(anim) = animations.animations.get(current_anim_index) {
            if let Some(frame) = anim.frames.get(anim.current_frame) {
                sprite.index = *frame as u32;
            }
        }
    }
}

fn animate(mut query: Query<(&mut TextureAtlasSprite, &mut Animations)>) {
    for (mut sprite, mut animations) in query.iter_mut() {
        if let Some(anim) = animations.get_animation_mut() {
            anim.current_frame += 1;
            if anim.current_frame >= anim.frames.len() {
                anim.current_frame = 0;
            }
        }
    }
}

fn animate_movement(mut query: Query<(&mut Animations, &Velocity, &Movement)>) {
    for (mut anims, vel, movement) in query.iter_mut() {
        let is_moving = (vel.0 as Vec2).distance_squared(Vec2::ZERO) > 0.0;
        // Compute movement direction
        let angle = movement.last_angle;
        let anim = match angle {
            0 if is_moving => 7,
            0 => 6,
            1 if is_moving => 3,
            1 => 2,
            2 if is_moving => 5,
            2 => 4,
            3 if is_moving => 1,
            3 => 0,
            _ => 0,
        };
        anims.set_animation(anim);
    }
}
