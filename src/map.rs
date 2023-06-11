use std::{time::Duration, fs::{self, create_dir_all, ReadDir}, path::PathBuf, io::ErrorKind};

use bevy::{prelude::*, time::Stopwatch};
use bevy_kira_audio::{AudioChannel, AudioControl, AudioInstance};
use lyon_geom::Point;

use crate::{sprites::{Timing, HitObjectTemplate, HitObjectAdditionalParams, OsuHitObjectType, slider::{OsuSliderParams, OsuSliderCurveType}, HitSoundRaw, HitObjectID, background::Background, SpriteType}, file_reader::{self, extract_archive}, GameState, GameStateEnum, audio::BeatmapMusicChannel};

const OSUPIXELS_WIDTH: f32 = 640.0;
const OSUPIXELS_HEIGHT: f32 = 480.0;

const BEATMAP_GENERAL_INTO_HEADER: &'static str = "[General]";
const BEATMAP_METADATA_HEADER: &'static str = "[Metadata]";
const BEATMAP_EVENTS_HEADER: &'static str = "[Events]";
const BEATMAP_HITOBJECT_HEADER: &'static str = "[HitObjects]";

#[derive(Component, Default, Debug)]
pub struct OsuBeatmapInfo {
    pub audio_filename: String
}

#[derive(Component, Default, Debug)]
pub struct OsuBeatmapMetadata {
    pub title: String,
    pub title_u: String,
    pub artist: String,
    pub artist_u: String,
    pub creator: String,
    pub version: String,
    pub source: String,
    pub tags: String,
    pub beatmap_id: u32,
    pub beatmapset_id: u32,
    pub path: PathBuf
}

#[derive(Component, Debug)]
pub struct OsuBeatmap {
    pub loaded: bool,
    pub running_time: Stopwatch,
    pub current_circle_index: usize,
    pub circles: Vec<HitObjectTemplate>,
    pub audio_handle: Handle<AudioInstance>,
    pub beatmap_info: OsuBeatmapInfo,
    pub beatmap_metadata: OsuBeatmapMetadata
}

pub struct OsuBeatmapPacksBriefed(pub Vec<OsuBeatmapMetadata>, pub PathBuf);

#[derive(Resource)]
pub struct OsuBeatmapPacksBriefedLoaded(pub Vec<OsuBeatmapPacksBriefed>);

#[derive(Resource)]
pub struct CurrentOsuBeatmap(pub OsuBeatmap);

pub struct MapLoadingWanted(pub PathBuf);

// pub fn load_first_avail_beatmap(
//     mut state: ResMut<GameState>,
//     mut map_event: EventWriter<MapLoadingWanted>
// ) {
//     fn no_beatmap_msg() {
//         warn!("No maps found in assets/maps! Load them by dragging zip to game window or manually unzip .osz to game folder");
//     }
//     let mut beatmap_path = PathBuf::new();
//     let beatmap_folders = match get_beatmaps_folder() {
//         Ok(dir) => dir,
//         Err(e) => {
//             error!("{}", e);
//             return
//         }
//     };
//     debug!("Beatmaps folder in game folder found");
//     debug!("Searching for folders in beatmaps folder...");
//     for beatmap_folder in beatmap_folders {
//         match beatmap_folder {
//             Ok(entry) => {
//                 trace!("{:?}", entry);
//                 let mut entry_path = entry.path().to_path_buf();
//                 entry_path.push("*.osu");
//                 beatmap_path = match file_reader::find_single(entry_path.to_str().unwrap()) {
//                     Ok(path) => {
//                         debug!("Found beatmap main file at {:?}", path);
//                         path
//                     },
//                     Err(_) => {
//                         no_beatmap_msg();
//                         return
//                     }
//                 }
//             },
//             Err(_) => {
//                 no_beatmap_msg();
//                 return
//             }
//         }
//     }
//     if beatmap_path.to_str().unwrap().is_empty() {
//         no_beatmap_msg();
//     }
//     else {
//         map_event.send(MapLoadingWanted(beatmap_path));
//     }
// }

fn get_beatmaps_folder() -> Result<ReadDir, std::io::Error> {
    return match fs::read_dir("assets/maps/") {
        Ok(dir) => Ok(dir),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                warn!("{:?}", create_dir_all("assets/maps"));
                get_beatmaps_folder()
            },
            e => Err(e.into())
        }
    }
}

fn load_beatmap_file(beatmap_path: PathBuf) -> Result<Vec<String>, &'static str> {
    let beatmap = match file_reader::load_file(beatmap_path) {
        Ok(f) => f,
        Err(_) => {
            let error = "Invalid beatmap file path!";
            error!(error);
            return Err(error);
        }
    };
    Ok(beatmap)
}

fn load_beatmap_general_info(beatmap: &Vec<String>) -> OsuBeatmapInfo {
    let beatmap_info_section = file_reader::get_section(&beatmap, BEATMAP_GENERAL_INTO_HEADER);
    let mut beatmap_info = OsuBeatmapInfo::default();
    for line in beatmap_info_section.iter() {
        let mut line = line.split(':');

        let left_field = line.next().expect("Invalid [General] format!");
        let right_field = line.next().expect("Invalid [General] format!");

        match left_field {
            "AudioFilename" => beatmap_info.audio_filename = right_field.trim().to_string(),
            _ => () // TODO - more info to parse
        }
    }
    beatmap_info
}

fn load_beatmap_metadata(beatmap: &Vec<String>) -> OsuBeatmapMetadata {
    let beatmap_metadata_section = file_reader::get_section(&beatmap, BEATMAP_METADATA_HEADER);
    let mut beatmap_metadata = OsuBeatmapMetadata::default();
    for line in beatmap_metadata_section.iter() {
        let mut line = line.split(':');

        let left_field = line.next().expect("Invalid [General] format!");
        let right_field = line.next().expect("Invalid [General] format!");

        match left_field {
            "Title" => beatmap_metadata.title = right_field.trim().to_string(),
            "TitleUnicode" => beatmap_metadata.title_u = right_field.trim().to_string(),
            "Artist" => beatmap_metadata.artist = right_field.trim().to_string(),
            "ArtistUnicode" => beatmap_metadata.artist_u = right_field.trim().to_string(),
            "Creator" => beatmap_metadata.creator = right_field.trim().to_string(),
            "Version" => beatmap_metadata.version = right_field.trim().to_string(),
            "Source" => beatmap_metadata.source = right_field.trim().to_string(),
            "Tags" => beatmap_metadata.tags = right_field.trim().to_string(),
            "BeatmapID" => beatmap_metadata.beatmap_id = right_field.parse::<u32>().ok().unwrap(),
            "BeatmapSetID" => beatmap_metadata.beatmapset_id = right_field.parse::<u32>().ok().unwrap(),
            _ => ()
        }
    }
    beatmap_metadata
}

pub fn load_beatmap(
    mut commands: Commands,
    sprites: Query<Entity, With<SpriteType>>,
    audio: Res<AudioChannel<BeatmapMusicChannel>>,
    asset_server: Res<AssetServer>,
    mut event_receiver: EventReader<MapLoadingWanted>,
    mut game_state: ResMut<GameState>
) {
    for event in event_receiver.iter() {
        if let Err(e) = load_beatmap_with_result(&mut commands, event.0.clone(), &sprites, &audio, &asset_server, &mut game_state) {
            error!("{}", e);
        }
    }
}

pub fn load_beatmap_with_result(
    commands: &mut Commands,
    beatmap_path: PathBuf,
    sprites: &Query<Entity, With<SpriteType>>,
    audio: &Res<AudioChannel<BeatmapMusicChannel>>,
    asset_server: &Res<AssetServer>,
    game_state: &mut ResMut<GameState>
) -> Result<(), &'static str> {
    if let GameStateEnum::Playing = game_state.0 {
        debug!("Stopping previous beatmap");
        audio.stop();
        game_state.0 = GameStateEnum::NothingPlaying;

        for sprite in sprites {
            commands.entity(sprite).despawn();
        }
    }

    let beatmap_root = beatmap_path.parent().unwrap().canonicalize().ok().unwrap();

    debug!("Loading beatmap from {:?}", beatmap_path);
    let beatmap = load_beatmap_file(beatmap_path.clone())?;
    debug!("Beatmap file loading successful");

    debug!("Loading beatmap general info");
    let beatmap_info = load_beatmap_general_info(&beatmap);
    debug!("Beatmap general info loaded successfully");
    trace!("{:#?}", beatmap_info);

    debug!("Loading beatmap metadata");
    let beatmap_metadata = load_beatmap_metadata(&beatmap);
    debug!("Beatmap metadata loaded successfully");
    trace!("{:#?}", beatmap_metadata);

    debug!("Loading beatmap events");
    let beatmap_events_section = file_reader::get_section(&beatmap, BEATMAP_EVENTS_HEADER);
    for line in beatmap_events_section.iter() {
        if !line.starts_with("//") {
            let mut line_spl = line.split(',');

            if line_spl.next().unwrap() == "0" {
                line_spl.next();

                let background_filename = beatmap_root.join(line_spl.next().unwrap().replace("\"", ""));
                let offset_x = line_spl.next().unwrap().parse::<u32>().ok().unwrap() as f32;
                let offset_y = line_spl.next().unwrap().parse::<u32>().ok().unwrap() as f32;
                trace!("{:?}", background_filename);

                let background = Background::set_background(&asset_server.load(background_filename), Transform::from_xyz(offset_x, offset_y, 0.0));
                commands.spawn(background);

                break;
            }
        }
    }
    debug!("Beatmap events loaded successfully");

    debug!("Creating hitcircle templates");
    let mut hitobjects: Vec<HitObjectTemplate> = Vec::new();
    let hitobjects_section = file_reader::get_section(&beatmap, BEATMAP_HITOBJECT_HEADER);
    for (line_i, line) in hitobjects_section.iter().enumerate() {
        let line_spl = &mut line.split(',');
        let mut x = 0.0;
        let mut y = 0.0;
        let mut time = 0;
        let mut circle_type = 0;
        let mut hitsound = 0;
        let mut params_raw = String::new();
        // slider data
        let mut slides_count = 0;
        let mut length = 0.0;

        for (pos, field) in line_spl.clone().into_iter().enumerate() {
            match pos {
                0 => x = field.parse::<f32>().ok().expect("HitObject x field not found!") - OSUPIXELS_WIDTH / 2.0,
                1 => y = -(field.parse::<f32>().ok().expect("HitObject y field not found!") - OSUPIXELS_HEIGHT / 2.0),
                2 => time = field.parse::<u32>().ok().expect("HitObject time field not found"),
                3 => circle_type = field.parse::<u32>().ok().expect("HitObject type field not found"),
                4 => hitsound = field.parse::<u8>().ok().expect("HitObject hitsound field not found"),
                _ => ()
            }
        }

        let mut params = HitObjectAdditionalParams::HitcircleParams;
        match circle_type.try_into() {
            Ok(OsuHitObjectType::HitCircle) => {
                params = HitObjectAdditionalParams::HitcircleParams;
            },
            Ok(OsuHitObjectType::Slider) => {
                for (pos, field) in line_spl.into_iter().enumerate() {
                    match pos {
                        5 => params_raw = field.to_string(),
                        6 => slides_count = field.parse::<u32>().ok().expect("HitObject slides field not found"),
                        7 => length = field.parse::<f32>().ok().expect("HitObject length field not found"),
                        _ => ()
                    }
                }

                let mut params_spl = params_raw.split('|');

                let curve_type = match params_spl.next().unwrap() {
                    "B" => OsuSliderCurveType::Bezier,
                    "C" => OsuSliderCurveType::CentripetalCatmullRom,
                    "L" => OsuSliderCurveType::Linear,
                    "P" => OsuSliderCurveType::PerfectCircle,
                    &_ => OsuSliderCurveType::Bezier
                };

                let mut curve_points = Vec::new();
                for point_p in params_spl {
                    let mut point_s = point_p.split(':');
                    let point_x = point_s.next().unwrap().parse::<f32>().ok().unwrap();
                    let point_y = point_s.next().unwrap().parse::<f32>().ok().unwrap();
                    let point = Point::new(point_x, point_y);
                    curve_points.push(point);
                }

                params = HitObjectAdditionalParams::SliderParams(
                    OsuSliderParams {
                        curve_type,
                        curve_points,
                        slides_count,
                        length,
                    }
                );
            },
            Ok(OsuHitObjectType::Spinner) => {
                error!("Hitobject type spinner not implemented!");
                continue
            },
            Ok(OsuHitObjectType::PerfectCircle) => {
                error!("Hitobject type perfect circle not implemented!");
            }
            Err(_) => error!("invalid hitobject type!")
        }

        let hitobject = HitObjectTemplate {
            id: HitObjectID(line_i as u64),
            timing: Timing(Duration::from_millis(time as u64)),
            position: Transform::from_xyz(x, y, 1000.0 - (line_i as f32 / 10.0)),
            hitsound: HitSoundRaw(hitsound),
            params
        };
        hitobjects.push(hitobject);
    }
    debug!("Created {} hitcircle templates successfully", hitobjects.len());

    let audio_handle = audio.play(
        asset_server.load(beatmap_root.join(&beatmap_info.audio_filename))
    ).handle();

    let mut beatmap = OsuBeatmap {
        loaded: true,
        running_time: Stopwatch::new(),
        current_circle_index: 0,
        circles: hitobjects,
        audio_handle,
        beatmap_info,
        beatmap_metadata
    };
    beatmap.beatmap_metadata.path = beatmap_path.clone();
    debug!("Beatmap loading successful");

    info!("Playing beatmap: {} - {}", beatmap.beatmap_metadata.artist, beatmap.beatmap_metadata.title);
    game_state.0 = GameStateEnum::Playing;
    commands.insert_resource(CurrentOsuBeatmap(beatmap));

    Ok(())
}

pub fn load_dnd_beatmap_archive(
    mut commands: Commands,
    mut dnd_events: EventReader<FileDragAndDrop>,
    mut map_event: EventWriter<MapLoadingWanted>
) {
    for ev in dnd_events.iter() {
        if let FileDragAndDrop::DroppedFile { window: _, path_buf } = ev {
            debug!("New file dragged onto window pointing to {:?}", path_buf);
            let dir_name = path_buf.file_stem().unwrap();
            let mut dst_dir = PathBuf::new();
            dst_dir.push("assets");
            dst_dir.push("maps");
            dst_dir.push(dir_name);
            match extract_archive(path_buf, &dst_dir) {
                Ok(()) => {
                    let mut beatmap_file_pattern = PathBuf::from(dst_dir);
                    beatmap_file_pattern.push("*.osu");
                    let beatmap_file = file_reader::find_single(beatmap_file_pattern.to_str().unwrap());
                    match beatmap_file {
                        Ok(path) => {
                            commands.remove_resource::<OsuBeatmapPacksBriefedLoaded>();
                            commands.init_resource::<OsuBeatmapPacksBriefedLoaded>();
                            map_event.send(MapLoadingWanted(path));
                        },
                        Err(e) => error!("{}", e)
                    };
                },
                Err(e) => error!("{}", e)
            };
        }
    }
}

impl FromWorld for OsuBeatmapPacksBriefedLoaded {
    fn from_world(_world: &mut World) -> Self {
        fn no_beatmap_msg() {
            warn!("No maps found in assets/maps! Load them by dragging zip to game window or manually unzip .osz to game folder");
        }
        let mut beatmaps: Vec<OsuBeatmapPacksBriefed> = Vec::new();
        let beatmap_folders = match get_beatmaps_folder() {
            Ok(dir) => dir,
            Err(e) => {
                error!("{}", e);
                return OsuBeatmapPacksBriefedLoaded(Vec::new())
            }
        };
        debug!("Beatmaps folder in game folder found");
        debug!("Searching for folders in beatmaps folder...");
        for beatmap_folder in beatmap_folders {
            match beatmap_folder {
                Ok(entry) => {
                    trace!("{:?}", entry);
                    let mut entry_path = entry.path().to_path_buf();
                    entry_path.push("*.osu");
                    beatmaps.push(match file_reader::find_all(entry_path.to_str().unwrap()) {
                        Some(beatmap_version_paths) => {
                            debug!("Found {} beatmap versions", beatmap_version_paths.len());
                            let mut beatmap_versions: Vec<OsuBeatmapMetadata> = Vec::new();
                            for beatmap_version_path in beatmap_version_paths.clone() {
                                match get_briefed_beatmap(beatmap_version_path.clone()) {
                                    Ok(meta) => {
                                        trace!("{:?}", meta);
                                        let mut meta = meta;
                                        meta.path = beatmap_version_path;
                                        beatmap_versions.push(meta);
                                    },
                                    Err(_) => ()
                                };
                            };
                            let beatmap_pack_path = beatmap_version_paths[0].parent().unwrap().to_path_buf();
                            let beatmap_pack = OsuBeatmapPacksBriefed(beatmap_versions, beatmap_pack_path);
                            beatmap_pack
                        },
                        None => OsuBeatmapPacksBriefed(Vec::<OsuBeatmapMetadata>::new(), PathBuf::new())
                    });
                },
                Err(_) => {
                    no_beatmap_msg();
                    return OsuBeatmapPacksBriefedLoaded(Vec::new())
                }
            }
        }
        OsuBeatmapPacksBriefedLoaded(beatmaps)
    }
}

fn get_briefed_beatmap(beatmap_path: PathBuf) -> Result<OsuBeatmapMetadata, &'static str> {
    let beatmap = load_beatmap_file(beatmap_path)?;
    Ok(load_beatmap_metadata(&beatmap))
}