use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform, Visibility, SceneRoot)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

const CAMERA_DISTANCE: f32 = 40.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));

    // Spawn player
    commands.spawn((
        Player,
        SceneRoot(asset_server.load("Buggy.glb#Scene0")),
        Transform {
            rotation: Quat::from_rotation_y(PI / 2.0),
            ..default()
        },
    ));
}

fn player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let speed = 2.0;

    if keyboard.pressed(KeyCode::KeyW) {
        player_transform.translation.z += speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyS) {
        player_transform.translation.z -= speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyA) {
        player_transform.translation.x += speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyD) {
        player_transform.translation.x -= speed * time.delta_secs();
    }
}
