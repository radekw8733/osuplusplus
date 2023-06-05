#[allow(unused_imports)]
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))] // disable linking on WASM and release builds
use bevy_dylib;
use bevy::{prelude::*, input::{mouse::MouseButtonInput, ButtonState}, window::{PrimaryWindow, PresentMode}, log::LogPlugin};
use bevy_tweening::{TweeningPlugin, Animator};
use map::CurrentOsuMap;
use sprites::{hitcircle::{OsuCircle, HitObjectID}, SpriteType, background::Background, HitObject};
use skin::SkinResources;

mod sprites;
mod skin;
mod map;
mod file_reader;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            #[cfg(debug_assertions)]
            filter: "warn,bevy_render=info,wgpu_hal=error,osuplusplus=trace".to_string(),
            ..Default::default()
        }).set(WindowPlugin {
            primary_window: Some(Window {
                title: "osu++".to_string(),
                fit_canvas_to_parent: true,
                present_mode: PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(TweeningPlugin)
        .init_resource::<SkinResources>()
        .add_startup_system(setup)
        .add_startup_system(map::load_first_avail_beatmap.after(setup))
        .add_system(mouse_click_event)
        .add_system(map::load_dnd_beatmap_archive)
        .add_system(running_map_loop.after(map::load_first_avail_beatmap))
        .add_system(OsuCircle::hitcircle_shown)
        .run();
}

fn setup(mut commands: Commands, skin: Res<SkinResources>) {
    let background = Background::setup_background(skin);
    commands.spawn(background);
    commands.spawn(Camera2dBundle::default());
}

fn mouse_click_event(
    mut mouse_event: EventReader<MouseButtonInput>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut circles: Query<(&SpriteType, &mut Transform, Entity, &Sprite, &mut Animator<Sprite>, &mut Animator<Transform>, &HitObjectID)>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let window = window.get_single().ok().unwrap();
    let (camera, camera_transform) = camera_q.single();

    if let Some(cursor) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        for e in mouse_event.iter() {
            if e.button == MouseButton::Left && e.state == ButtonState::Pressed {
                let mut circles_vec: Vec<(&SpriteType, Mut<Transform>, Entity, &Sprite, Mut<Animator<Sprite>>, Mut<Animator<Transform>>, &HitObjectID)> = circles.iter_mut().collect();
                // sort over circle age
                circles_vec.sort_by(|a, b| {
                    a.6.0.cmp(&b.6.0)
                });
                // keep searching in ordered circles
                for circle in circles_vec {
                    if OsuCircle::click_event(cursor, circle).is_ok() {
                        break
                    }
                }
            }
        }
    }
}

fn running_map_loop(
    mut commands: Commands,
    skin: Res<SkinResources>,
    time: Res<Time>,
    map: Option<ResMut<CurrentOsuMap>>
) {
    if map.is_some() {
        let map = &mut map.unwrap().0;
        map.running_time.tick(time.delta());
        if !map.running_time.paused() {
            if map.running_time.elapsed() >= map.circles[map.current_circle_index].timing.0 {
                let circle = OsuCircle::new_hitobject(&map.circles[map.current_circle_index], skin);
                map.current_circle_index += 1;
                commands.spawn(circle);
            }
        }
    }
}