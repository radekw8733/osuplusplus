use std::{path::PathBuf, fs::create_dir_all};

use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::file_reader;

const ASSETS_ROOT: &'static str = "assets/";
const SKIN_DIRECTORY: &'static str = "skin/";
const SKIN_SPRITES_DIRECTORY: &'static str = "sprites/";
const SKIN_SOUNDS_DIRECTORY: &'static str = "sounds/";

#[derive(Resource)]
pub struct SkinResources {
    pub sprites: SkinSprites,
    pub sounds: SkinSounds
}

#[derive(Component)]
pub struct SkinSprites {
    pub hitcircle_handle: Handle<Image>,
    pub background_handle: Handle<Image>,
    pub cursor_handle: Handle<Image>
}

#[derive(Component)]
pub struct SkinSounds {
    // pub drums: SkinSoundsSet,
    pub normal: SkinSoundsSet,
    // pub soft: SkinSoundsSet
}

#[derive(Default, Component)]
pub struct SkinSoundsSet {
    pub hitnormal: Handle<AudioSource>,
    pub hitclap: Handle<AudioSource>,
    pub hitfinish: Handle<AudioSource>,
    pub hitwhistle: Handle<AudioSource>,
    pub slidertick: Handle<AudioSource>,
    pub sliderslide: Handle<AudioSource>,
    pub sliderwhistle: Handle<AudioSource>,
}

impl FromWorld for SkinResources {
    fn from_world(world: &mut World) -> Self {
        let mut asset_server = world.get_resource_mut::<AssetServer>().unwrap();

        let skin_search_path: PathBuf = [ASSETS_ROOT, SKIN_DIRECTORY].iter().collect();
        if !skin_search_path.exists() {
            warn!("{:?}", create_dir_all(&skin_search_path));
        }
        let skin_dir = file_reader::find_single_dir(skin_search_path.to_str().unwrap());
        match skin_dir {
            Ok(_) => (),
            Err(_) => error!("Any skin hasn't been found!")
        }

        let sprites = Self::load_sprites(&mut asset_server);
        let sounds = Self::load_sounds(&mut asset_server);

        SkinResources {
            sprites,
            sounds
        }
    }
}

impl SkinResources {
    fn load_sprites(asset_server: &mut Mut<AssetServer>) -> SkinSprites {
        let sprites_dir: PathBuf = [SKIN_DIRECTORY, SKIN_SPRITES_DIRECTORY].iter().collect();
        let hitcircle_handle = asset_server.load(sprites_dir.join("hitcircle.png"));
        let background_handle = asset_server.load(sprites_dir.join("background.jpg"));
        let cursor_handle = asset_server.load(sprites_dir.join("cursor.png"));

        SkinSprites {
            hitcircle_handle,
            background_handle,
            cursor_handle
        }
    }

    fn load_sounds(asset_server: &mut Mut<AssetServer>) -> SkinSounds {
        // let drums = Self::load_soundset("drum", asset_server);
        let normal = Self::load_soundset("normal", asset_server);
        // let soft = Self::load_soundset("soft", asset_server);

        SkinSounds {
            // drums,
            normal,
            // soft
        }
    }

    fn load_soundset(set: &str, asset_server: &mut Mut<AssetServer>) -> SkinSoundsSet {
        let sounds_dir: PathBuf = [ASSETS_ROOT, SKIN_DIRECTORY, SKIN_SOUNDS_DIRECTORY].iter().collect();
        let mut soundset = SkinSoundsSet::default();
        // TODO - accept only audio files, not every extension
        soundset.hitnormal = asset_server.load(file_reader::find_single(sounds_dir.join(format!("{}-hitnormal.*", set)).to_str().unwrap()).ok().unwrap().canonicalize().ok().unwrap());
        soundset.hitclap = asset_server.load(file_reader::find_single(sounds_dir.join(format!("{}-hitclap.*", set)).to_str().unwrap()).ok().unwrap().canonicalize().ok().unwrap());
        soundset.hitfinish = asset_server.load(file_reader::find_single(sounds_dir.join(format!("{}-hitfinish.*", set)).to_str().unwrap()).ok().unwrap().canonicalize().ok().unwrap());
        soundset.hitwhistle = asset_server.load(file_reader::find_single(sounds_dir.join(format!("{}-hitwhistle.*", set)).to_str().unwrap()).ok().unwrap().canonicalize().ok().unwrap());
        soundset.slidertick = asset_server.load(file_reader::find_single(sounds_dir.join(format!("{}-slidertick.*", set)).to_str().unwrap()).ok().unwrap().canonicalize().ok().unwrap());
        soundset.sliderslide = asset_server.load(file_reader::find_single(sounds_dir.join(format!("{}-sliderslide.*", set)).to_str().unwrap()).ok().unwrap().canonicalize().ok().unwrap());
        soundset.sliderwhistle = asset_server.load(file_reader::find_single(sounds_dir.join(format!("{}-sliderwhistle.*", set)).to_str().unwrap()).ok().unwrap().canonicalize().ok().unwrap());

        soundset
    }
}