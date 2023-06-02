use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, Tween, lens::{SpriteColorLens, TransformScaleLens}, TweenCompleted, EaseMethod, EaseFunction, AnimatorState, Delay};

use crate::skin::SkinResources;

use super::SpriteType;

pub const HITCIRCLE_SIZE: f32 = 150.0;
pub const HITCIRCLE_FADE_DURATION_MILLIS: u64 = 500;

#[derive(Component, Clone, Copy, Debug)]
pub struct CircleID(pub u64);

#[derive(Component, Copy, Clone, Debug)]
pub struct Timing(pub Duration);

#[repr(u64)]
enum AnimationCompletedType {
    Shown,
    Hidden
}

impl TryFrom<u64> for AnimationCompletedType {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            x if x == AnimationCompletedType::Hidden as u64 => Ok(AnimationCompletedType::Hidden),
            x if x == AnimationCompletedType::Shown as u64 => Ok(AnimationCompletedType::Shown),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OsuCircleTemplate {
    pub id: CircleID,
    pub transform: Transform,
    pub timing: Timing
}

#[derive(Bundle)]
pub struct OsuCircle {
    pub sprite_type: SpriteType,
    pub timing: Timing,
    pub sprite_animator: Animator<Sprite>,
    pub transform_animator: Animator<Transform>,
    pub hitcircle_sprite: SpriteBundle,
    pub id: CircleID,
}

impl OsuCircle {
    pub fn click_event(
        cursor: Vec2,
        mut circle: (&SpriteType, Mut<Transform>, Entity, &Sprite, Mut<Animator<Sprite>>, Mut<Animator<Transform>>, &CircleID),
    ) -> Result<(), ()> {
        let x_dif = (cursor.x - circle.1.translation.x).powi(2);
        let y_dif = (cursor.y - circle.1.translation.y).powi(2);
        let dist = (x_dif + y_dif).sqrt();
        if dist < HITCIRCLE_SIZE / 3.0 {
            debug!("{:?} CLICKED!", circle.0);
            
            let cur_color = circle.3.color;
            let fadeout_duration = HITCIRCLE_FADE_DURATION_MILLIS as f32 * cur_color.a() / 4.0;
            let fadeout = Tween::new(
                EaseMethod::Linear,
                Duration::from_millis(fadeout_duration as u64),
                SpriteColorLens {
                    start: cur_color,
                    end: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.0 }
                }
            ).with_completed_event(AnimationCompletedType::Hidden as u64);
            circle.4.set_tweenable(fadeout);
            circle.5.state = AnimatorState::Playing;
            return Ok(());
        }
        return Err(());
    }

    pub fn new_circle(id: CircleID, temp: OsuCircleTemplate, skin: Res<SkinResources>) -> OsuCircle {
        let sprite = SpriteBundle {
            transform: temp.transform,
            texture: skin.hitcircle_handle.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(150.0, 150.0)),
                color: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.0 },
                ..Default::default()
            },
            ..Default::default()
        };

        let fadein = Tween::new(
            EaseMethod::Linear,
            Duration::from_millis(HITCIRCLE_FADE_DURATION_MILLIS),
            SpriteColorLens {
                start: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.0 },
                end: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }
            }
        );
        let delay: Delay<Sprite> = Delay::new(Duration::from_millis(HITCIRCLE_FADE_DURATION_MILLIS / 2));
        let fadeout = Tween::new(
            EaseMethod::Linear,
            Duration::from_millis(HITCIRCLE_FADE_DURATION_MILLIS / 2),
            SpriteColorLens {
                start: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 },
                end: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.0 }
            }
        );
        let scaleup = Tween::new(
            EaseFunction::CubicOut,
            Duration::from_millis(HITCIRCLE_FADE_DURATION_MILLIS * 2),
            TransformScaleLens {
                start: Vec3::new(1.0, 1.0, 1.0),
                end: Vec3::new(1.8, 1.8, 1.0)
            }
        );
        let seq = fadein
            .with_completed_event(AnimationCompletedType::Shown as u64)
            .then(delay)
            .then(fadeout.with_completed_event(AnimationCompletedType::Hidden as u64));

        OsuCircle {
            sprite_type: SpriteType::Hitcircle(temp.id),
            timing: temp.timing,
            sprite_animator: Animator::new(seq),
            transform_animator: Animator::new(scaleup).with_state(bevy_tweening::AnimatorState::Paused),
            hitcircle_sprite: sprite,
            id,
        }
    }

    pub fn hitcircle_shown(
        mut events: EventReader<TweenCompleted>,
        query: Query<(Entity, &SpriteType), With<SpriteType>>,
        mut commands: Commands
    ) {
        for event in events.iter() {
            let object = event.entity;
            let anim_type: AnimationCompletedType = event.user_data.try_into().expect("Invalid AnimationCompletedType!");
            for query_s in query.iter() {
                if object == query_s.0 {
                    if let SpriteType::Hitcircle(CircleID(_circle)) = query_s.1 {
                        match anim_type {
                            AnimationCompletedType::Shown => (),
                            AnimationCompletedType::Hidden => {
                                commands.entity(object).despawn();
                            }
                        }
                    }
                }
            }
        }
    }
}
