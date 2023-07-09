
use std::sync::mpsc::Receiver;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(startup_system)
        .add_system(state_system)
        .run();
}

#[derive(Component, Debug)]
enum PowerState {
    On,
    Off
}
#[derive(Component, Debug)]
struct Link {
    sender: Option<Transistor>,
    receiver: Option<Transistor>
}

#[derive(Component, Debug)]
struct Transistor {
    state: PowerState
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2dBundle::default());
    // Quad
    
    commands.spawn((
        MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 100.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::DARK_GRAY)),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        
        ..default()
    },
    Transistor { state: PowerState::On }
));

}

fn state_system(mut query: Query<&mut Transistor>) {
    
    for mut transistor in &mut query {
        println!("{:?}", transistor.state);
        match transistor.state {
            PowerState::On => true,
            PowerState::Off => false,
        };

        if matches!(transistor.state, PowerState::Off) {
            transistor.state = PowerState::On;
        } else {
            transistor.state = PowerState::Off;
        }
    }
}

fn startup_system(mut commands: Commands) {
    commands.spawn_batch(vec![
        (
            Transistor {
                state: PowerState::Off,
            }
        ),

    ])
}