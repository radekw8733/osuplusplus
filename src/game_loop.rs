use std::time::Duration;

use bevy::{prelude::{Res, Assets, ResMut, Commands, EventReader, With, Query, Transform, Entity, Camera, GlobalTransform, MouseButton, Mut, KeyCode, Vec2}, time::Time, input::{mouse::MouseButtonInput, ButtonState, keyboard::KeyboardInput}, window::{PrimaryWindow, Window}, sprite::Sprite};
use bevy_kira_audio::{AudioInstance, PlaybackState, AudioTween, AudioChannel};
use bevy_tweening::Animator;

use crate::{skin::SkinResources, GameState, map::CurrentOsuBeatmap, GameStateEnum, sprites::{hitcircle::OsuCircle, HitObject, SpriteType, HitObjectID, HitSoundRaw}, audio::HitObjectSoundChannel};

pub fn main_loop(
    mut commands: Commands,
    skin: Res<SkinResources>,
    time: Res<Time>,
    game_state: Res<GameState>,
    mut audio_assets: ResMut<Assets<AudioInstance>>,
    map: Option<ResMut<CurrentOsuBeatmap>>
) {
    if map.is_some() {
        let map = &mut map.unwrap().0;

        if audio_assets.get(&map.audio_handle).is_some() {
            let music = audio_assets.get_mut(&map.audio_handle).unwrap();

            match game_state.0 {
                GameStateEnum::Playing => {
                    map.running_time.tick(time.delta());
                    if map.running_time.elapsed() >= map.circles[map.current_circle_index].timing.0 {
                        let circle = OsuCircle::new_hitobject(&map.circles[map.current_circle_index], skin);
                        map.current_circle_index += 1;
                        commands.spawn(circle);
                    }

                    if let PlaybackState::Paused { position: _ } = music.state() {
                        music.resume(AudioTween::linear(Duration::ZERO));
                    }
                },
                GameStateEnum::Paused => {
                    if let PlaybackState::Playing { position: _ } = music.state() {
                        music.pause(AudioTween::linear(Duration::ZERO));
                    }
                },
                _ => ()
            }
        }
    }
}

pub fn input_event_handler(
    mut mouse_events: EventReader<MouseButtonInput>,
    mut keyboard_events: EventReader<KeyboardInput>,
    window: Query<&Window, With<PrimaryWindow>>,
    skin: Res<SkinResources>,
    audio_channel: Res<AudioChannel<HitObjectSoundChannel>>,
    mut circles: Query<(&SpriteType, &mut Transform, Entity, &Sprite, &mut Animator<Sprite>, &mut Animator<Transform>, &HitObjectID, &HitSoundRaw)>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let window = window.get_single().ok().unwrap();
    let (camera, camera_transform) = camera_q.single();

    let mut circle_sorter = |cursor: Vec2| {
        let mut circles_vec: Vec<(&SpriteType, Mut<Transform>, Entity, &Sprite, Mut<Animator<Sprite>>, Mut<Animator<Transform>>, &HitObjectID, &HitSoundRaw)> = circles.iter_mut().collect();
        // sort over circle age
        circles_vec.sort_by(|a, b| {
            a.6.0.cmp(&b.6.0)
        });
        // keep searching in ordered circles
        for circle in circles_vec {
            if OsuCircle::click_event(cursor, &audio_channel, &skin, circle).is_ok() {
                break
            }
        }
    };

    if let Some(cursor) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        for e in mouse_events.iter() {
            if e.button == MouseButton::Left && e.state == ButtonState::Pressed {
                circle_sorter(cursor);
            }
        }
        for kev in keyboard_events.iter() {
            if let ButtonState::Pressed = kev.state {
                if let Some(key) = kev.key_code {
                    match key {
                        KeyCode::S | KeyCode::D => {
                            circle_sorter(cursor);
                        },
                        _ => ()
                    }
                }
            }
        }
    }
}