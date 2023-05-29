use bevy::prelude::Component;

use self::hitcircle::CircleID;

pub mod hitcircle;
pub mod background;

#[derive(Component)]
pub enum SpriteType {
    Hitcircle(CircleID),
    Background,
}