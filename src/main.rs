use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3d {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLight {
        intensity: 1500.0,
        shadows_enabled: true,
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Player rectangle (cube)
    commands.spawn((
        (
            Mesh3d(meshes.add(Mesh::from(shape::Box::new(1.0, 0.2, 1.0)))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6).into())),
            Transform::from_xyz(0.0, 0.1, 0.0),
        ),
        Player,
    ));

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(shape::Plane::from_size(5.0).into())),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3).into())),
    ));
}

fn player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let speed = 2.0;

    if keyboard.pressed(KeyCode::KeyW) {
        player_transform.translation.z -= speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyS) {
        player_transform.translation.z += speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyA) {
        player_transform.translation.x -= speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyD) {
        player_transform.translation.x += speed * time.delta_secs();
    }
}
