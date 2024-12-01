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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Player rectangle (cube)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 0.2, 1.0))),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.1, 0.0),
            ..default()
        },
        Player,
    ));

    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

fn player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let speed = 2.0;

    if keyboard.pressed(KeyCode::W) {
        player_transform.translation.z -= speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        player_transform.translation.z += speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        player_transform.translation.x -= speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        player_transform.translation.x += speed * time.delta_seconds();
    }
}
