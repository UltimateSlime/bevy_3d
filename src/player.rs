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
    mut query: Query<(Entity, &mut LinearVelocity, &Transform), With<Player>>,
    spatial_query: SpatialQuery,
) {
    let Ok((entity, mut velocity, transform)) = query.single_mut() else { return; };


    let speed = 5.0;
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
    if keyboard.pressed(KeyCode::KeyS) { direction.z += 1.0; }
    if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0; }

    velocity.x = direction.normalize_or_zero().x * speed;
    velocity.z = direction.normalize_or_zero().z * speed;

    // 足元に地面があるか確認
    let grounded = spatial_query.cast_ray(
        transform.translation,              // レイの開始点 (プレイヤーの位置)
        Dir3::NEG_Y,               // 下方向    
        1.1,                    // 距離 (1.1 = カプセルの半径+少し余裕)
        true,                           // 固体のみ
        &SpatialQueryFilter::from_excluded_entities(vec![entity]),  // 自分自身を除外
    ).is_some();
    
    // ジャンプ追加
    if keyboard.just_pressed(KeyCode::Space) && grounded {
        velocity.y = 8.0; // ジャンプの高さを調整
    }

}

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else { return; };
    let Ok(mut camera_transform) = camera_query.single_mut() else { return; };

    // カメラをプレイヤーの後ろに配置
    let offset = Vec3::new(0.0, 5.0, 10.0);
    camera_transform.translation = player_transform.translation + offset;

    // カメラをプレイヤーの方に向ける
    camera_transform.look_at(player_transform.translation, Vec3::Y);

}