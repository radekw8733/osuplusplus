use bevy::prelude::Component;

use self::hitcircle::CircleID;

pub mod hitcircle;
pub mod background;

#[derive(Component, Debug)]
pub enum SpriteType {
    Hitcircle(CircleID),
    Background,
}