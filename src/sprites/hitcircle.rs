use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};
use bevy_tweening::{Animator, Tween, lens::{SpriteColorLens, TransformScaleLens}, TweenCompleted, EaseMethod, EaseFunction, AnimatorState, Delay};

use crate::{skin::SkinResources, audio::HitObjectSoundChannel};

use super::{SpriteType, HitObject, Timing, HitObjectTemplate, HitSoundRaw, HitObjectID};

pub const HITCIRCLE_SIZE: f32 = 150.0;
pub const HITCIRCLE_FADE_DURATION_MILLIS: u64 = 500;

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
    id: HitObjectID,
    sprite_type: SpriteType,
    timing: Timing,
    hitsound: HitSoundRaw,
    sprite_animator: Animator<Sprite>,
    transform_animator: Animator<Transform>,
    hitcircle_sprite: SpriteBundle
}

impl HitObject for OsuCircle {
    fn new_hitobject(temp: &HitObjectTemplate, skin: Res<SkinResources>) -> OsuCircle {
        let sprite = SpriteBundle {
            transform: temp.position,
            texture: skin.sprites.hitcircle_handle.clone(),
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
            id: temp.id,
            sprite_type: SpriteType::Hitcircle,
            timing: temp.timing,
            hitsound: temp.hitsound,
            sprite_animator: Animator::new(seq),
            transform_animator: Animator::new(scaleup).with_state(bevy_tweening::AnimatorState::Paused),
            hitcircle_sprite: sprite
        }
    }
}

impl OsuCircle {
    pub fn click_event(
        cursor: Vec2,
        audio_channel: &Res<AudioChannel<HitObjectSoundChannel>>,
        skin: &Res<SkinResources>,
        mut circle: (&SpriteType, Mut<Transform>, Entity, &Sprite, Mut<Animator<Sprite>>, Mut<Animator<Transform>>, &HitObjectID, &HitSoundRaw),
    ) -> Result<(), ()> {
        let x_dif = (cursor.x - circle.1.translation.x).powi(2);
        let y_dif = (cursor.y - circle.1.translation.y).powi(2);
        let dist = (x_dif + y_dif).sqrt();
        if dist < HITCIRCLE_SIZE / 3.0 {
            trace!("{:?} at ({}, {}) clicked!", circle.0, circle.1.translation.x, circle.1.translation.y);

            Self::play_sound(circle.7, audio_channel, skin);
            
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
                    if let SpriteType::Hitcircle = query_s.1 {
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

    fn play_sound(hitsound_type: &HitSoundRaw, audio_channel: &Res<AudioChannel<HitObjectSoundChannel>>, skin: &Res<SkinResources>) {
        // clap
        if hitsound_type.0 & 0b1000 == 0b1000 {
            audio_channel.play(skin.sounds.normal.hitclap.clone());
        }
        // finish
        if hitsound_type.0 & 0b100 == 0b100 {
            audio_channel.play(skin.sounds.normal.hitfinish.clone());
        }
        // whistle
        if hitsound_type.0 & 0b10 == 0b10 {
            audio_channel.play(skin.sounds.normal.hitwhistle.clone());
        }
        // normal
        if hitsound_type.0 & 0b1 == 0b1 || hitsound_type.0 & 0 == 0 {
            audio_channel.play(skin.sounds.normal.hitnormal.clone());
        }
    }
}