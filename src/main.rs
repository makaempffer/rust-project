use rand::Rng;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::{ PresentMode }};


fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "I am a window!".into(),
            resolution: (800., 800.).into(),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
      }))
        .add_startup_system(setup)
        .add_system(mouse_system)
        .run();
}

#[derive(Component)]
struct Velocity { velocity: Vec3 }
#[derive(Component)]
struct Entity { }
#[derive(Component)]
struct Cell { }

fn spawn_cell(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Quad::new(Vec2 {x: 20.0, y: 20.0}).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3 { x: (rand::thread_rng().gen_range(-400..400) as f32), y: (rand::thread_rng().gen_range(-400..400) as f32), z: (0.0) }),
        ..default()
    }).insert(Cell { });
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

//rand::thread_rng().gen_range(0..800) as f32

fn mouse_system(commands: Commands, meshes: ResMut<Assets<Mesh>>, materials: ResMut<Assets<ColorMaterial>>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        spawn_cell(commands, meshes, materials)
    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
    }
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down
    }
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        // Either the left or the right button was just pressed
    }
}