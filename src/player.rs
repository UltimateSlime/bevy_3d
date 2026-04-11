use bevy::prelude::*;
use avian3d::prelude::*;
use crate::camera::CameraAngle; 

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerAnimations {
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub jump: AnimationNodeIndex,
    pub graph: Handle<AnimationGraph>,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut graph = AnimationGraph::new();

    let idle = graph.add_clip(asset_server.load("models/player.glb#Animation0"),1.0, graph.root);
    let walk = graph.add_clip(asset_server.load("models/player.glb#Animation2"),1.0, graph.root);
    let jump= graph.add_clip(asset_server.load("models/player.glb#Animation1"),1.0, graph.root);

    let graph_handle = graphs.add(graph);

    commands.insert_resource(PlayerAnimations {
        idle,
        walk,
        jump,
        graph: graph_handle.clone(),
   });

    commands.spawn((
        Transform::from_xyz(0.0, 2.0, 0.0),
        RigidBody::Dynamic,
        Collider::capsule(0.3,1.1),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        Player,
    )).with_child((
        SceneRoot(asset_server.load("models/player.glb#Scene0")),
        Transform::from_xyz(0.0, -0.85, 0.0),
    ));
}

pub fn setup_player_animation(
    mut commands: Commands,
    animations: Res<PlayerAnimations>,
    mut players: Query<Entity, Added<AnimationPlayer>>
) {
    for entity in &mut players {
        commands.entity(entity).insert(AnimationGraphHandle(animations.graph.clone()));};
}


pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut LinearVelocity, &Transform), With<Player>>,
    spatial_query: SpatialQuery,
    camera_query: Query<&CameraAngle, With<Camera3d>>,
) {
    let Ok((entity, mut velocity, transform)) = query.single_mut() else { return; };
    let Ok(angle) = camera_query.single() else { return; };


    let speed = 5.0;
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
    if keyboard.pressed(KeyCode::KeyS) { direction.z += 1.0; }
    if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0; }

    // カメラの向きに合わせて移動方向を回転
    let yaw_rotation = Quat::from_rotation_y(angle.yaw());
    let direction = yaw_rotation * direction.normalize_or_zero();

    velocity.x = direction.x * speed;
    velocity.z = direction.z * speed;

    // 足元に地面があるか確認
    let grounded = spatial_query.cast_shape(
        &Collider::cylinder(0.35, 0.0),
        transform.translation,              // レイの開始点 (プレイヤーの位置)
        Quat::IDENTITY,
        Dir3::NEG_Y,               // 下方向    
        &ShapeCastConfig::from_max_distance(1.5),
        &SpatialQueryFilter::from_excluded_entities(vec![entity]),  // 自分自身を除外
    ).is_some();
    
    // ジャンプ追加
    if keyboard.just_pressed(KeyCode::Space) && grounded {
        velocity.y = 8.0; // ジャンプの高さを調整
    }

}

pub fn update_animation(
    animations: Res<PlayerAnimations>,
    player_query: Query<(&LinearVelocity, &Transform, Entity), With<Player>>,
    spatial_query: SpatialQuery,
    mut anim_players: Query<&mut AnimationPlayer>,
    mut current_anim: Local<Option<AnimationNodeIndex>>,
) {
    let Ok((velocity, transform, entity)) = player_query.single() else { return; };

    let grounded = spatial_query.cast_shape(
        &Collider::cylinder(0.35, 0.0),
        transform.translation,
        Quat::IDENTITY,
        Dir3::NEG_Y,
        &ShapeCastConfig::from_max_distance(1.5),
        &SpatialQueryFilter::from_excluded_entities(vec![entity]),
    ).is_some();

    let is_moving = (velocity.x.abs() > 0.1) || (velocity.z.abs() > 0.1);
    let is_jumping = !grounded;  

    let next_anim = if is_jumping {
        animations.jump
    } else if is_moving {
        animations.walk
    } else {
        animations.idle
    };

    if *current_anim != Some(next_anim) {
        *current_anim = Some(next_anim);
        for mut player in &mut anim_players {
            player.stop_all();

            if next_anim == animations.jump {
                player.play(next_anim);
            } else {
                player.play(next_anim).repeat();
            }
        }
    }
    
}