use bevy::{sprite::{SpriteBundle, Sprite}, prelude::{Bundle, Res, Transform}};

use crate::skin::SkinResources;

use super::SpriteType;

#[derive(Bundle)]
pub struct Background {
    pub sprite_type: SpriteType,
    pub sprite: SpriteBundle
}

impl Background {
    pub fn setup_background(skin: Res<SkinResources>) -> Background {
        let sprite = SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                color: bevy::prelude::Color::RgbaLinear { red: 0.3, green: 0.3, blue: 0.3, alpha: 1.0 },
                ..Default::default()
            },
            texture: skin.background_handle.clone(),
            ..Default::default()
        };
        Background {
            sprite_type: SpriteType::Background,
            sprite
        }
    }
}