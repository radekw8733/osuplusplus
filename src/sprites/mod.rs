use std::time::Duration;

use bevy::prelude::{Component, Res, Transform};

use crate::skin::SkinResources;

use self::slider::OsuSliderParams;

pub mod hitcircle;
pub mod slider;
pub mod background;

#[derive(Component, Debug)]
pub enum SpriteType {
    Hitcircle,
    Slider,
    Background,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct HitObjectID(pub u64);

pub enum OsuHitObjectType {
    HitCircle = 1,
    Slider = 2,
    Spinner = 4,
    PerfectCircle = 64,
}

impl TryFrom<u32> for OsuHitObjectType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if OsuHitObjectType::HitCircle as u32 & x == OsuHitObjectType::HitCircle as u32 => Ok(OsuHitObjectType::HitCircle),
            x if OsuHitObjectType::Slider as u32 & x == OsuHitObjectType::Slider as u32 => Ok(OsuHitObjectType::Slider),
            x if OsuHitObjectType::Spinner as u32 & x == OsuHitObjectType::Spinner as u32 => Ok(OsuHitObjectType::Spinner),
            x if OsuHitObjectType::PerfectCircle as u32 & x == OsuHitObjectType::PerfectCircle as u32 => Ok(OsuHitObjectType::PerfectCircle),
            _ => Err(()),
        }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Timing(pub Duration);

#[derive(Component, Copy, Clone, Debug)]
pub struct HitSoundRaw(pub u8);

#[derive(Component, Clone, Debug)]
pub struct HitObjectTemplate {
    pub id: HitObjectID,
    pub position: Transform,
    pub timing: Timing,
    pub hitsound: HitSoundRaw,
    pub params: HitObjectAdditionalParams,
    // pub hitsample: HitSample
}

#[derive(Clone, Debug)]
pub enum HitObjectAdditionalParams {
    HitcircleParams,
    SliderParams(OsuSliderParams),
    // SpinnerParams {
    //     endtime: Timing
    // }
}

// #[derive(Default, Copy, Clone, Debug)]
// pub struct HitSample {
//     pub normal_set: u32,
//     pub addition_set: u32,
//     pub index: u32,
//     pub vol: u32,
// }

pub trait HitObject {
    fn new_hitobject(temp: &HitObjectTemplate, skin: Res<SkinResources>) -> Self;
}