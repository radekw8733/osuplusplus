use audio::{HitObjectSoundChannel, BeatmapMusicChannel};
#[allow(unused_imports)]
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))] // disable linking on WASM and release builds
use bevy_dylib;
use bevy::{prelude::*, window::PresentMode, log::LogPlugin};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::{AudioPlugin, AudioApp};
use bevy_tweening::{TweeningPlugin};
use map::OsuBeatmapPacksBriefedLoaded;
use sprites::hitcircle::OsuCircle;
use skin::SkinResources;

mod audio;
mod sprites;
mod skin;
mod map;
mod file_reader;
mod ui;
mod game_loop;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum GameStateEnum {
    #[default]
    NothingPlaying,
    Playing,
    Paused,
}

#[derive(Resource, Default)]
pub struct GameState(GameStateEnum);

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
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TweeningPlugin)
        .add_plugin(AudioPlugin)
        .add_event::<map::MapLoadingWanted>()
        .init_resource::<OsuBeatmapPacksBriefedLoaded>()
        .init_resource::<SkinResources>()
        .init_resource::<GameState>()
        .add_audio_channel::<BeatmapMusicChannel>()
        .add_audio_channel::<HitObjectSoundChannel>()
        .add_startup_system(setup)
        .add_system(map::load_beatmap)
        .add_system(map::load_dnd_beatmap_archive)
        .add_system(ui::display_loader_window)
        .add_system(game_loop::main_loop)
        .add_system(game_loop::input_event_handler)
        .add_system(OsuCircle::hitcircle_shown)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}