use bevy::prelude::*;

pub const HITCIRCLE_SIZE: f32 = 150.0;

#[derive(Component)]
pub struct CircleID(pub String);

#[derive(Component)]
pub struct MoveTimer(pub Timer);

#[derive(Bundle)]
pub struct OsuCircle {
    pub id: CircleID,
    pub timer: MoveTimer,
    pub sprite: SpriteBundle
}