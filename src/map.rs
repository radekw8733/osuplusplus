use std::{time::Duration, fs, path::PathBuf};

use bevy::{prelude::*, time::Stopwatch};

use crate::{sprites::hitcircle::{Timing, OsuCircleTemplate, CircleID}, file_reader::{self, extract_archive}};

const OSUPIXELS_WIDTH: f32 = 640.0;
const OSUPIXELS_HEIGHT: f32 = 480.0;

const BEATMAP_GENERAL_INTO_HEADER: &'static str = "[General]";
const BEATMAP_METADATA_HEADER: &'static str = "[Metadata]";
const BEATMAP_HITOBJECT_HEADER: &'static str = "[HitObjects]";

#[derive(Component, Default, Debug)]
pub struct OsuMapInfo {
    pub audio_filename: String
}

#[derive(Component, Default, Debug)]
pub struct OsuMapMetadata {
    pub title: String,
    pub title_u: String,
    pub artist: String,
    pub artist_u: String,
    pub creator: String,
    pub version: String,
    pub source: String,
    pub tags: String,
    pub beatmap_id: u32,
    pub beatmapset_id: u32
}

#[derive(Component, Debug)]
pub struct OsuMap {
    pub loaded: bool,
    pub running_time: Stopwatch,
    pub current_circle_index: usize,
    pub circles: Vec<OsuCircleTemplate>,
    pub beatmap_info: OsuMapInfo,
    pub beatmap_metadata: OsuMapMetadata
}

#[derive(Resource)]
pub struct CurrentOsuMap(pub OsuMap);

pub fn load_first_avail_beatmap(mut commands: Commands) {
    fn no_beatmap_msg() {
        warn!("No maps found in assets/maps! Load them by dragging zip to game window or manually unzip .osz to game folder");
    }
    // TODO: Error handling when no maps
    let mut beatmap_path = PathBuf::new();
    let beatmap_folders = match fs::read_dir("assets/maps/") {
        Ok(dir) => dir,
        Err(e) => {
            error!("{}", e);
            return
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
                beatmap_path = match file_reader::find_single(entry_path.to_str().unwrap()) {
                    Ok(path) => {
                        debug!("Found beatmap main file at {:?}", path);
                        path
                    },
                    Err(_) => {
                        no_beatmap_msg();
                        return
                    }
                }
            },
            Err(_) => {
                no_beatmap_msg();
                return
            }
        }
    }
    if beatmap_path.to_str().unwrap().is_empty() {
        no_beatmap_msg();
    }
    else {
        load_beatmap(&mut commands, beatmap_path)
    }
    
}

pub fn load_beatmap(commands: &mut Commands, beatmap_path: PathBuf) {
    debug!("Loading beatmap from {:?}", beatmap_path);
    let beatmap = match file_reader::load_file(beatmap_path) {
        Ok(f) => f,
        Err(_) => {
            error!("Invalid beatmap file path!");
            return;
        }
    };
    debug!("Beatmap file loading successful");

    trace!("Loading beatmap general info");
    let beatmap_info_section = file_reader::get_section(&beatmap, BEATMAP_GENERAL_INTO_HEADER);
    let mut beatmap_info = OsuMapInfo::default();
    for line in beatmap_info_section.iter() {
        let mut line = line.split(':');

        let left_field = line.next().expect("Invalid [General] format!");
        let right_field = line.next().expect("Invalid [General] format!");

        match left_field {
            "AudioFilename" => beatmap_info.audio_filename = right_field.trim().to_string(),
            _ => () // TODO - more info to parse
        }
    }
    trace!("Beatmap general info loaded successfully");
    trace!("{:#?}", beatmap_info);

    trace!("Loading beatmap metadata");
    let beatmap_metadata_section = file_reader::get_section(&beatmap, BEATMAP_METADATA_HEADER);
    let mut beatmap_metadata = OsuMapMetadata::default();
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
            _ => () // TODO - more info to parse
        }
    }
    trace!("Beatmap metadata loaded successfully");
    trace!("{:#?}", beatmap_metadata);

    trace!("Creating hitcircle templates");
    let mut circles: Vec<OsuCircleTemplate> = Vec::new();
    let hitobjects_section = file_reader::get_section(&beatmap, BEATMAP_HITOBJECT_HEADER);
    for (line_i, line) in hitobjects_section.iter().enumerate() {
        let line = line.split(',');
        let mut x = 0.0;
        let mut y = 0.0;
        let mut time = 0;
        // let mut circle_type = 0;

        for (pos, field) in line.into_iter().enumerate() {
            match pos {
                0 => x = field.parse::<f32>().ok().expect("HitObject x field not found!") - OSUPIXELS_WIDTH / 2.0,
                1 => y = -(field.parse::<f32>().ok().expect("HitObject y field not found!") - OSUPIXELS_HEIGHT / 2.0),
                2 => time = field.parse::<u32>().ok().expect("HitObject time field not found"),
                _ => ()
            }
        }

        let circle = OsuCircleTemplate {
            id: CircleID(line_i as u64),
            timing: Timing(Duration::from_millis(time as u64)),
            transform: Transform::from_xyz(x, y, 1000.0 - (line_i as f32 / 10.0))
        };
        circles.push(circle);
    }
    trace!("Created {} hitcircle templates successfully", circles.len());

    let beatmap = OsuMap {
        loaded: true,
        running_time: Stopwatch::new(),
        current_circle_index: 0,
        circles,
        beatmap_info,
        beatmap_metadata
    };
    debug!("Beatmap loading successful");
    info!("Playing beatmap: {} - {}", beatmap.beatmap_metadata.artist, beatmap.beatmap_metadata.title);
    commands.insert_resource(CurrentOsuMap(beatmap));
}

pub fn load_dnd_beatmap_archive(mut commands: Commands, mut dnd_events: EventReader<FileDragAndDrop>) {
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
                        Ok(path) => load_beatmap(&mut commands, path),
                        Err(e) => error!("{}", e)
                    }
                },
                Err(e) => error!("{}", e)
            };
            
        }
    }
}