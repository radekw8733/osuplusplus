use bevy::prelude::*;

#[derive(Resource)]
pub struct SkinResources {
    pub hitcircle_texture: Handle<ColorMaterial>,
    pub hitcircle_image: Handle<Image>,
    pub background_texture: Handle<ColorMaterial>,
    pub cursor_texture: Handle<ColorMaterial>
}

impl FromWorld for SkinResources {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let hitcircle_handle = asset_server.load("sprites/hitcircle.png");
        let background_handle = asset_server.load("sprites/background.jpg");
        let cursor_handle = asset_server.load("sprites/cursor.png");

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        SkinResources {
            hitcircle_texture: materials.add(hitcircle_handle.clone().into()),
            hitcircle_image: hitcircle_handle,
            background_texture: materials.add(background_handle.into()),
            cursor_texture: materials.add(cursor_handle.into())
        }
    }
}