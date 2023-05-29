use bevy::prelude::*;

#[derive(Resource)]
pub struct SkinResources {
    pub hitcircle_handle: Handle<Image>,
    pub background_handle: Handle<Image>,
    pub cursor_handle: Handle<Image>
}

impl FromWorld for SkinResources {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let hitcircle_handle = asset_server.load("sprites/hitcircle.png");
        let background_handle = asset_server.load("sprites/background.jpg");
        let cursor_handle = asset_server.load("sprites/cursor.png");
        SkinResources {
            hitcircle_handle,
            background_handle,
            cursor_handle
        }
    }
}