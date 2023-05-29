use core::array::from_fn;

#[allow(unused_imports)]
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))] // disable linking on WASM and release builds
use bevy_dylib;
use bevy::{prelude::*, input::{mouse::MouseButtonInput, ButtonState}, window::PrimaryWindow, log::LogPlugin};
use sprites::{hitcircle::{CircleID, MoveTimer, OsuCircle}, SpriteType, background::Background};
use rand::Rng;
use skin::SkinResources;

mod sprites;
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

    let mut rng = rand::thread_rng();
    let circles: [OsuCircle; 20] = from_fn(|_| {
        OsuCircle::new_circle(
            format!("circle{}", rng.gen_range(0..10000)),
            &skin,
            Transform::from_xyz(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0), 1.0)
        )
    });
    let background = Background::setup_background(skin);

    commands.spawn(background);
    commands.spawn_batch(circles)
}

fn mouse_click_event(
    mut mouse_event: EventReader<MouseButtonInput>,
    window: Query<&Window, With<PrimaryWindow>>,
    circles: Query<(&SpriteType, &mut Transform, Entity)>,
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
                    if let SpriteType::Hitcircle(id) = circle.0 {
                        OsuCircle::click_event(cursor, circle, id, &skin, &mut commands);
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