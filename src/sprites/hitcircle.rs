use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, Tween, EaseFunction, lens::SpriteColorLens, TweenCompleted};
use rand::Rng;

use crate::skin::SkinResources;

use super::SpriteType;

pub const HITCIRCLE_SIZE: f32 = 150.0;

#[derive(Component, Debug)]
pub struct CircleID(pub String);

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

#[derive(Bundle)]
pub struct OsuCircle {
    pub sprite_type: SpriteType,
    pub animator: Animator<Sprite>,
    pub sprite: SpriteBundle
}

impl OsuCircle {
    pub fn click_event(
        cursor: Vec2,
        circle: (&SpriteType, &Transform, Entity),
        id: &CircleID,
        skin: &Res<SkinResources>,
        commands: &mut Commands,
    ) {
        let x_dif = (cursor.x - circle.1.translation.x).powi(2);
        let y_dif = (cursor.y - circle.1.translation.y).powi(2);
        let dist = (x_dif + y_dif).sqrt();
        if dist < HITCIRCLE_SIZE / 3.0 {
            let mut rng = rand::thread_rng();
            let transform = Transform::from_xyz(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0), 1.0);
    
            debug!("{:?} CLICKED!", id);
            commands.entity(circle.2).despawn();
            commands.spawn(Self::new_circle(format!("circle{}", rng.gen_range(0..10000)), &skin, transform));
        }
    }

    pub fn new_circle(id: String, skin: &Res<SkinResources>, transform: Transform) -> OsuCircle {
        let sprite = SpriteBundle {
            transform,
            texture: skin.hitcircle_handle.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(150.0, 150.0)),
                color: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.0 },
                ..Default::default()
            },
            ..Default::default()
        };

        let fadein = Tween::new(
            EaseFunction::SineInOut,
            Duration::from_secs(1),
            SpriteColorLens {
                start: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.0 },
                end: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }
            }
        );
        let fadeout = Tween::new(
            EaseFunction::SineInOut,
            Duration::from_secs(1),
            SpriteColorLens {
                start: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 },
                end: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.0 }
            }
        );
        let seq = fadein
            .with_completed_event(AnimationCompletedType::Shown as u64)
            .then(fadeout.with_completed_event(AnimationCompletedType::Hidden as u64));

        OsuCircle {
            sprite_type: SpriteType::Hitcircle(CircleID(id)),
            animator: Animator::new(seq),
            sprite
        }
    }

    pub fn hitcircle_shown(
        mut events: EventReader<TweenCompleted>,
        query: Query<(Entity, &SpriteType), With<SpriteType>>,
        skin: Res<SkinResources>,
        mut commands: Commands
    ) {
        let mut rng = rand::thread_rng();
        for event in events.iter() {
            let object = event.entity;
            let anim_type: AnimationCompletedType = event.user_data.try_into().expect("Invalid AnimationCompletedType!");
            for query_s in query.iter() {
                if object == query_s.0 {
                    if let SpriteType::Hitcircle(CircleID(_circle)) = query_s.1 {
                        match anim_type {
                            AnimationCompletedType::Shown => {
                                // commands.spawn(Self::new_circle(
                                //     format!("circle{}", rng.gen_range(0..10000)),
                                //     &skin,
                                //     Transform::from_xyz(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0), 1.0)
                                // ));
                            }
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
