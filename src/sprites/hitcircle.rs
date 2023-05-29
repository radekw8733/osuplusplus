use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::skin::SkinResources;

use super::SpriteType;

pub const HITCIRCLE_SIZE: f32 = 150.0;

#[derive(Component, Debug)]
pub struct CircleID(pub String);

#[derive(Component)]
pub struct MoveTimer(pub Timer);

#[derive(Bundle)]
pub struct OsuCircle {
    pub sprite_type: SpriteType,
    pub timer: MoveTimer,
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
                ..Default::default()
            },
            ..Default::default()
        };
        OsuCircle {
            sprite_type: SpriteType::Hitcircle(CircleID(id)),
            timer: MoveTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)),
            sprite
        }
    }
}
