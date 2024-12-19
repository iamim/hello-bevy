use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform, Visibility, SceneRoot)]
struct Player;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        // .add_systems(Update, player_movement)
        .add_systems(Update, player_apply_force)
        .run();
}

const CAMERA_DISTANCE: f32 = 40.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(4.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(4.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    // Dynamic physics object with a collision shape and initial angular velocity
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Collider::sphere(1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 4.0, 0.0),
        ExternalForce::new(Vec3::ZERO),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, CAMERA_DISTANCE, -CAMERA_DISTANCE).looking_at(Vec3::ZERO, Vec3::Z),
    ));
}

fn player_apply_force(
    mut query: Query<(&mut ExternalForce, &mut MeshMaterial3d<StandardMaterial>), With<Player>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let (mut player_external_force, mut material) = query.single_mut();

    if keyboard.just_pressed(KeyCode::KeyE) {
        player_external_force.apply_force(Vec3::Z * 10.0);
        material.0 = materials.add(Color::BLACK);
    }

    if keyboard.just_released(KeyCode::KeyE) {
        player_external_force.clear();
        material.0 = materials.add(Color::WHITE);
    }

    if keyboard.just_pressed(KeyCode::KeyS) {
        player_external_force.apply_force(Vec3::X * 10.0);
        material.0 = materials.add(Color::BLACK);
    }

    if keyboard.just_released(KeyCode::KeyS) {
        player_external_force.clear();
        material.0 = materials.add(Color::WHITE);
    }

    if keyboard.just_pressed(KeyCode::KeyD) {
        player_external_force.apply_force(-Vec3::Z * 10.0);
        material.0 = materials.add(Color::BLACK);
    }

    if keyboard.just_released(KeyCode::KeyD) {
        player_external_force.clear();
        material.0 = materials.add(Color::WHITE);
    }

    if keyboard.just_pressed(KeyCode::KeyF) {
        player_external_force.apply_force(-Vec3::X * 10.0);
        material.0 = materials.add(Color::BLACK);
    }

    if keyboard.just_released(KeyCode::KeyF) {
        player_external_force.clear();
        material.0 = materials.add(Color::WHITE);
    }
}
