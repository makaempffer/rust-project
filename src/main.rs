
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(startup_system)
        .add_system(state_system)
        .run();
}

#[derive(Component)]
struct Transistor {
    state: bool
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    // Quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 100.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });

}

fn state_system(mut query: Query<&mut Transistor>) {
    for transistor in &mut query {
        println!("{}", transistor.state);
    }
}

fn startup_system(mut commands: Commands) {
    commands.spawn_batch(vec![
        (
            Transistor {
                state: false,
            }
        ),

    ])
}