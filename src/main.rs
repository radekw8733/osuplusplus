use bevy::{prelude::{App, Commands, ResMut, Assets, Mesh, shape::Circle, Color, Camera2dBundle, Transform, Res, Query, Bundle, Component, Handle}, DefaultPlugins, sprite::{ColorMaterial, MaterialMesh2dBundle}, time::Time};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_around)
        .run();
}

#[derive(Component)]
struct CircleID(String);

#[derive(Bundle)]
struct OsuCircle {
    id: CircleID,
    mesh: MaterialMesh2dBundle<ColorMaterial>
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    let mesh = MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(20.0).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_xyz(10.0, 20.0, 0.0),
        ..Default::default()
    };
    let circle = OsuCircle {
        id: CircleID("circle".to_string()),
        mesh
    };

    commands.spawn(circle);
}

fn move_around(mut sprites: Query<(&CircleID, &mut Transform, &mut Handle<ColorMaterial>)>, time: Res<Time>) {
    let mut rng = rand::thread_rng();
    for mut sprite in &mut sprites {
        if sprite.0.0 == "circle" {
            // transform.1.translation.x += 10.0 * time.delta_seconds();
            sprite.1.translation.x = rng.gen_range(0..100) as f32;
            sprite.1.translation.y = rng.gen_range(0..100) as f32;
        }
    }
}