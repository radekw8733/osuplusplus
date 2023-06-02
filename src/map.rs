use std::{time::Duration, fs, path::PathBuf};

use bevy::{prelude::*, time::Stopwatch};

use crate::{sprites::hitcircle::{Timing, OsuCircleTemplate, CircleID}, file_reader};

const OSUPIXELS_WIDTH: f32 = 640.0;
const OSUPIXELS_HEIGHT: f32 = 480.0;

#[derive(Component)]
pub struct OsuMap {
    pub loaded: bool,
    pub running_time: Stopwatch,
    pub current_circle_index: usize,
    pub circles: Vec<OsuCircleTemplate>
}

#[derive(Resource)]
pub struct CurrentOsuMap(pub OsuMap);

pub fn load_map(mut commands: Commands) {
    fn no_map_msg() {
        warn!("No maps found in assets/maps! Load them by dragging zip to game window or manually unzip .osz to game folder");
    }
    // TODO: Error handling when no maps
    let mut beatmap_path = PathBuf::new();
    let beatmap_folders = fs::read_dir("assets/maps/").expect("assets/maps not found!");
    for beatmap_folder in beatmap_folders {
        match beatmap_folder {
            Ok(entry) => {
                let mut entry_path = entry.path().to_path_buf();
                entry_path.push("*.osu");
                beatmap_path = match file_reader::find_single(entry_path.to_str().unwrap()) {
                    Ok(path) => path,
                    Err(_) => {
                        no_map_msg();
                        return
                    }
                }
            },
            Err(_) => {
                no_map_msg();
                return
            }
        }
    }

    let beatmap = match file_reader::load_file(beatmap_path) {
        Ok(f) => f,
        Err(_) => {
            info!("No maps found in assets/maps!");
            return;
        }
    };
    let hitobject_header = String::from("[HitObjects]");
    let mut circles: Vec<OsuCircleTemplate> = Vec::new();
    let mut hitobjects_header_index = 0;

    for (index, line) in beatmap.iter().enumerate() {
        if line.contains(&hitobject_header) {
            hitobjects_header_index = index;
            break;
        }
    }

    let hitobject_entries = &beatmap[hitobjects_header_index+1..beatmap.len()];
    for (line_i, line) in hitobject_entries.iter().enumerate() {
        if line == "" {
            break
        }

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

    let map = OsuMap {
        loaded: true,
        running_time: Stopwatch::new(),
        current_circle_index: 0,
        circles
    };
    commands.insert_resource(CurrentOsuMap(map));
}