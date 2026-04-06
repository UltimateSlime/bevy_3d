use bevy::prelude::*;
use avian3d::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.6, 0.9))),
        Transform::from_xyz(0.0, 2.0, 0.0),
        RigidBody::Dynamic,
        Collider::capsule(0.4, 1.0),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        Player,
    ));
}

pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut LinearVelocity, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut velocity) = query.single_mut() else { return; };


    let speed = 5.0;
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) { direction.z -= 1.0; }
    if keyboard.pressed(KeyCode::ArrowDown) { direction.z += 1.0; }
    if keyboard.pressed(KeyCode::ArrowLeft) { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }

    velocity.x = direction.normalize_or_zero().x * speed;
    velocity.z = direction.normalize_or_zero().z * speed;
}
