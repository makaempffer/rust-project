use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(startup_system)
        .add_system(mouse_click_system)
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

fn spawn_component_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_windows.single();

    let width = window.resolution.width();
    let height = window.resolution.height();

    let (x, y) = match window.position {
        WindowPosition::At(v) => (v.x as f32, v.y as f32),
        _ => (0., 0.),
    };


    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
        commands.spawn((
            MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 100.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::DARK_GRAY)),
            transform: Transform::from_translation(Vec3::new(position.x - width/2.0, position.y - height / 2.0, 0.)),
            
            ..default()
        },
        Transistor { state: PowerState::Off }
    ));
    } else {
        println!("Can't spawn if cursor is not in the game window.");
    }
    
}

fn state_system(mut query: Query<&mut Transistor>) {
    for transistor in &mut query {
        match transistor.state {
            PowerState::On => true,
            PowerState::Off => false,
        };
    }
}

fn mouse_click_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<Input<MouseButton>>
) {
    if mouse_button_input.pressed(MouseButton::Left) {

        info!("left mouse currently pressed");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        spawn_component_system(commands, meshes, materials, q_windows);
        info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
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