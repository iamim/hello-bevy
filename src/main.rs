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
    mut query: Query<&mut ExternalForce, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut player_external_force = query.single_mut();

    if keyboard.just_pressed(KeyCode::KeyE) {
        player_external_force.apply_force(Vec3::Z * 10.0);
    }
    if keyboard.just_released(KeyCode::KeyE) {
        player_external_force.apply_force(Vec3::ZERO);
    }
}

// fn player_movement(
//     mut query: Query<&mut Transform, With<Player>>,
//     keyboard: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
// ) {
//     let mut player_transform = query.single_mut();
//     let speed = 2.0;
//
//     if keyboard.pressed(KeyCode::KeyE) {
//         player_transform.translation.z += speed * time.delta_secs();
//     }
//     if keyboard.pressed(KeyCode::KeyD) {
//         player_transform.translation.z -= speed * time.delta_secs();
//     }
//     if keyboard.pressed(KeyCode::KeyS) {
//         player_transform.translation.x += speed * time.delta_secs();
//     }
//     if keyboard.pressed(KeyCode::KeyF) {
//         player_transform.translation.x -= speed * time.delta_secs();
//     }
// }
