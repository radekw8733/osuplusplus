use std::time::Duration;

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
    let hitobject_header = String::from("[HitObjects]");
    let map_text = file_reader::load_file("assets/maps/map0.osu");
    let mut circles: Vec<OsuCircleTemplate> = Vec::new();
    let mut hitobjects_header_index = 0;

    for (index, line) in map_text.iter().enumerate() {
        if line.contains(&hitobject_header) {
            hitobjects_header_index = index;
            break;
        }
    }

    let hitobject_entries = &map_text[hitobjects_header_index+1..map_text.len()];
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