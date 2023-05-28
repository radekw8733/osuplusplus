use std::time::Duration;

#[allow(unused_imports)]
#[cfg(debug_assertions)]
use bevy_dylib;
use bevy::{prelude::*, input::{mouse::MouseButtonInput, ButtonState}, window::PrimaryWindow, log::LogPlugin};
use hitcircle::{CircleID, MoveTimer, OsuCircle};
use rand::Rng;
use skin::SkinResources;

use crate::hitcircle::HITCIRCLE_SIZE;

mod hitcircle;
mod skin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            #[cfg(debug_assertions)]
            filter: "warn,bevy_render=info,osuplusplus=debug".to_string(),
            ..Default::default()
        }))
        .init_resource::<SkinResources>()
        .add_startup_system(setup)
        .add_system(mouse_click_event)
        .add_system(move_around)
        .run();
}

fn setup(mut commands: Commands, skin: Res<SkinResources>) {
    commands.spawn(Camera2dBundle::default());

    let circle = new_circle(&skin, Transform::default());

    commands.spawn(circle);
}

fn new_circle(skin: &Res<SkinResources>, transform: Transform) -> OsuCircle {
    let sprite = SpriteBundle {
        transform,
        texture: skin.hitcircle_image.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(150.0, 150.0)),
            ..Default::default()
        },
        ..Default::default()
    };
    hitcircle::OsuCircle {
        id: CircleID("circle1".to_string()),
        timer: MoveTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)),
        sprite
    }
}

fn mouse_click_event(
    mut mouse_event: EventReader<MouseButtonInput>,
    window: Query<&Window, With<PrimaryWindow>>,
    circles: Query<(&CircleID, &mut Transform, Entity)>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    skin: Res<SkinResources>,
    mut commands: Commands
) {
    let window = window.get_single().ok().unwrap();
    let (camera, camera_transform) = camera_q.single();

    if let Some(cursor) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        for e in mouse_event.iter() {
            if e.button == MouseButton::Left && e.state == ButtonState::Pressed {
                for circle in circles.iter() {
                    let x_dif = (cursor.x - circle.1.translation.x).powi(2);
                    let y_dif = (cursor.y - circle.1.translation.y).powi(2);
                    let dist = (x_dif + y_dif).sqrt();
                    debug!("dist: {:}", dist);
                    if dist < HITCIRCLE_SIZE / 3.0 {
                        let mut rng = rand::thread_rng();
                        let transform = Transform::from_xyz(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0), 0.0);

                        debug!("CLICKED!");
                        commands.entity(circle.2).despawn();
                        commands.spawn(new_circle(&skin, transform));
                    }
                }
            }
        }
    }
}

fn move_around(mut circles: Query<(&CircleID, &mut Transform, &mut MoveTimer)>, time: Res<Time>) {
    let mut rng = rand::thread_rng();
    for mut circle in &mut circles {
        circle.2.0.tick(time.delta());
        if circle.0.0 == "circle1" && circle.2.0.finished() {
            circle.1.translation.x += 10.0 * time.delta_seconds();
            circle.1.translation.x = rng.gen_range(0..100) as f32;
            circle.1.translation.y = rng.gen_range(0..100) as f32;
        }
    }
}