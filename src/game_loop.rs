use std::time::Duration;

use bevy::{prelude::{Res, Assets, ResMut, Commands}, time::Time};
use bevy_kira_audio::{AudioInstance, PlaybackState, AudioTween};

use crate::{skin::SkinResources, GameState, map::CurrentOsuBeatmap, GameStateEnum, sprites::{hitcircle::OsuCircle, HitObject}};

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