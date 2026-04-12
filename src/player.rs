use bevy::prelude::*;
use avian3d::prelude::*;
use crate::camera::CameraAngle; 

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerAnimations {
    pub idle: AnimationNodeIndex,
    pub walking: AnimationNodeIndex,
    pub jumping: AnimationNodeIndex,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Component, PartialEq, Debug, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    Jumping, 
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut graph = AnimationGraph::new();

    let idle = graph.add_clip(asset_server.load("models/player.glb#Animation0"),1.0, graph.root);
    let walking = graph.add_clip(asset_server.load("models/player.glb#Animation2"),1.0, graph.root);
    let jumping= graph.add_clip(asset_server.load("models/player.glb#Animation1"),1.0, graph.root);

    let graph_handle = graphs.add(graph);

    commands.insert_resource(PlayerAnimations {
        idle,
        walking,
        jumping,
        graph: graph_handle.clone(),
   });

    commands.spawn((
        Transform::from_xyz(0.0, 2.0, 0.0),
        RigidBody::Dynamic,
        Collider::capsule(0.3,1.1),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        Player,
        PlayerState::Idle,
    )).with_child((
        SceneRoot(asset_server.load("models/player.glb#Scene0")),
        Transform::from_xyz(0.0, -0.85, 0.0),
    ));
}

pub fn setup_player_animation(
    mut commands: Commands,
    animations: Res<PlayerAnimations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        commands.entity(entity).insert(AnimationGraphHandle(animations.graph.clone()));
        player.play(animations.idle).repeat();
    };
 
}


pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut LinearVelocity, &mut Transform, &mut PlayerState), With<Player>>,
    spatial_query: SpatialQuery,
    camera_query: Query<&CameraAngle, With<Camera3d>>,
) {
    let Ok((entity, mut velocity,mut transform, mut state)) = query.single_mut() else { return; };
    let Ok(angle) = camera_query.single() else { return; };


    let speed = if keyboard.pressed(KeyCode::ShiftLeft) {
        10.0
    } else {
        5.0
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
    if keyboard.pressed(KeyCode::KeyS) { direction.z += 1.0; }
    if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0; }

    // カメラの向きに合わせて移動方向を回転
    let yaw_rotation = Quat::from_rotation_y(angle.yaw());
    let direction = yaw_rotation * direction.normalize_or_zero();

    // 移動方向にプレイヤーを向かせる
    if direction.length_squared() > 0.01 {
        let target_rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
        transform.rotation = transform.rotation.slerp(target_rotation, 0.2);
    }

    // 足元に地面があるか確認
    let grounded = spatial_query.cast_shape(
        &Collider::cylinder(0.05, 0.0),
        transform.translation,              // レイの開始点 (プレイヤーの位置)
        Quat::IDENTITY,
        Dir3::NEG_Y,               // 下方向    
        &ShapeCastConfig::from_max_distance(1.1),
        &SpatialQueryFilter::from_excluded_entities(vec![entity]),  // 自分自身を除外
    ).is_some();
    
    if grounded {
        velocity.x = direction.x * speed;
        velocity.z = direction.z * speed;
    } else {
        // 空中にいる場合は水平移動を減速
        velocity.x *= 0.99;
        velocity.z *= 0.99;
    }


    let has_input = direction.length_squared() > 0.0;
    let is_moving = velocity.x.abs() > 0.1 || velocity.z.abs() > 0.1;

    // PlayerStateを更新
    *state = if !grounded {
        PlayerState::Jumping
    } else if has_input && is_moving {
        PlayerState::Walking
    } else {
        PlayerState::Idle
    };

    if keyboard.just_pressed(KeyCode::Space) && grounded {
        velocity.y = 8.0; // ジャンプの高さを調整
    }

}

pub fn update_animation(
    animations: Res<PlayerAnimations>,
    player_query: Query<&PlayerState, With<Player>>,
    mut anim_players: Query<&mut AnimationPlayer>,
    mut current_anim: Local<Option<AnimationNodeIndex>>,
) {
    let Ok(state) = player_query.single() else { return; };

    let next_anim = match state {
        PlayerState::Idle => animations.idle,
        PlayerState::Walking => animations.walking,
        PlayerState::Jumping => animations.jumping,
    };

    if *current_anim != Some(next_anim) {
        *current_anim = Some(next_anim);
        for mut player in &mut anim_players {
            player.stop_all();

            if *state == PlayerState::Jumping {
                player.play(next_anim).seek_to(19.0 / 30.0);
            } else {
                player.play(next_anim).repeat();
            }
    
       };
    }
}