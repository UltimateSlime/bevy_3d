use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use avian3d::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Resource, PartialEq, Clone, Copy)]
pub enum CameraMode {
    TPS,
    FPS,
}

#[derive(Resource)]
pub struct CameraAngle {
    yaw: f32,
    pitch: f32,
}

impl Default for CameraAngle{
    fn default() -> Self {
        Self { yaw: 0.0, pitch: 0.3 }
    }
}

impl CameraAngle {
    pub fn add_yaw(&mut self, delta: f32) {
        self.yaw -= delta;
    }
    pub fn add_pitch(&mut self, delta: f32) {
        self.pitch = (self.pitch - delta).clamp(-0.5, 1.4);
    }
    pub fn yaw(&self) -> f32 { self.yaw }
    pub fn pitch(&self) -> f32 { self.pitch }
}


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

pub fn switch_camera_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<CameraMode>,
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        *mode = match *mode {
            CameraMode::TPS => CameraMode::FPS,
            CameraMode::FPS => CameraMode::TPS,
        };
    }
}

pub fn mouse_look(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut angle: ResMut<CameraAngle>,
) {
    let sensitivity = 0.003;
    angle.add_yaw(mouse_motion.delta.x * sensitivity);
    angle.add_pitch(mouse_motion.delta.y * sensitivity);
}

pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut LinearVelocity, &Transform), With<Player>>,
    spatial_query: SpatialQuery,
    angle : Res<CameraAngle>,
) {
    let Ok((entity, mut velocity, transform)) = query.single_mut() else { return; };


    let speed = 5.0;
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
    if keyboard.pressed(KeyCode::KeyS) { direction.z += 1.0; }
    if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0; }

    // カメラの向きに合わせて移動方向を回転
    let yaw_rotation = Quat::from_rotation_y(angle.yaw());
    let direction = yaw_rotation * direction.normalize_or_zero();

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
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mode: Res<CameraMode>,
    angle: Res<CameraAngle>,
    spatial_query: SpatialQuery,
) {
    let Ok((player_entity, player_transform)) = player_query.single() else { return; };
    let Ok(mut camera_transform) = camera_query.single_mut() else { return; };

    let rotation = Quat::from_rotation_y(angle.yaw())
        * Quat::from_rotation_x(angle.pitch());

    match *mode {
        CameraMode::TPS => {
            let ideal_offset = rotation * Vec3::new(0.0, 5.0, 10.0);
            let ideal_pos = player_transform.translation + ideal_offset;

            // プレイヤー位置から理想位置へレイを飛ばす
            let direction = Dir3::new(ideal_offset.normalize()).unwrap();
            let distance = ideal_offset.length();

            let actual_pos = match spatial_query.cast_ray(
                player_transform.translation,
                direction,
                distance,
                true,
                &SpatialQueryFilter::from_excluded_entities(vec![player_entity]),
            ) {
                Some(hit) => {
                    // なにかにあたったらその少し手前に置く
                    player_transform.translation + ideal_offset.normalize() * (hit.distance - 0.3).max(0.1)
                }
                None => ideal_pos, // なにもなければ理想の位置
            };

            camera_transform.translation = actual_pos;
            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
        CameraMode::FPS => {
            let offset = Vec3::new(0.0, 1.6, 0.0);
            camera_transform.translation = player_transform.translation + offset;
            camera_transform.rotation = rotation;
        }
    }
}

