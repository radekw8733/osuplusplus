use bevy::{sprite::{SpriteBundle, Sprite}, prelude::{Transform, Image, Handle, Bundle}};

use super::SpriteType;

#[derive(Bundle)]
pub struct Background {
    pub sprite_type: SpriteType,
    pub sprite: SpriteBundle
}

impl Background {
    pub fn set_background(image: &Handle<Image>, offset: Transform) -> Background {
        let sprite = SpriteBundle {
            transform: offset,
            sprite: Sprite {
                color: bevy::prelude::Color::RgbaLinear { red: 0.3, green: 0.3, blue: 0.3, alpha: 1.0 },
                ..Default::default()
            },
            texture: image.clone(),
            ..Default::default()
        };
        Background {
            sprite_type: SpriteType::Background,
            sprite
        }
    }
}