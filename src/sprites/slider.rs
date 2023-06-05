use bevy::prelude::{Component, Res};
use lyon_geom::euclid::{Point2D, UnknownUnit};

use super::{HitObject, SpriteType, HitObjectAdditionalParams, HitObjectTemplate};

use crate::SkinResources;

#[derive(Component, Copy, Clone, Debug)]
pub enum OsuSliderCurveType {
    Bezier,
    CentripetalCatmullRom,
    Linear,
    PerfectCircle
}

#[derive(Clone, Debug)]
pub struct OsuSliderParams {
    pub curve_type: OsuSliderCurveType,
    pub curve_points: Vec<Point2D<f32, UnknownUnit>>,
    pub slides_count: u32,
    pub length: f32
}

pub struct OsuSlider {
    sprite_type: SpriteType,
    slider_type: OsuSliderCurveType,
    params: OsuSliderParams,
}

impl HitObject for OsuSlider {
    fn new_hitobject(temp: &HitObjectTemplate, skin: Res<SkinResources>) -> OsuSlider {
        if let HitObjectAdditionalParams::SliderParams(params) = &temp.params {
            OsuSlider {
                sprite_type: SpriteType::Slider(temp.id),
                slider_type: params.curve_type,
                params: params.clone(),
            }
        }
        else {
            panic!("invalid hitobject params!");
        }
    }
}