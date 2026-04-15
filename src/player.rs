use crate::camera::CameraAngle;
use avian3d::prelude::*;
use bevy::prelude::*;

pub const PLAYER_RADIUS: f32 = 0.3;
pub const PLAYER_HEIGHT: f32 = 1.2; // 全高 = HEIGHT + RADIUSx2 
pub const PLAYER_CROUCH_HEIGHT: f32 = 0.55; // 全高 = CROUCH_HEIGHT + RADIUSx2 
pub const PLAYER_SPEED: f32 = 5.0;
pub const PLAYER_DASH_SPEED: f32 = 10.0;
pub const PLAYER_CROUCH_SPEED: f32 = 5.0;
pub const JUMP_VELOCITY: f32 = 8.0;
pub const GROUNDED_CAST_DISTANCE: f32 = PLAYER_HEIGHT / 2.0 + PLAYER_RADIUS + 0.05;
pub const CAMERA_FPS_HEIGHT: f32 = 1.6; // 目の高さ・モデル依存
pub const CAMERA_CROUCH_OFFSET: f32 = -1.0; // しゃがみ時のオフセット・モデル依存
pub const GRAVITY: f32 = -9.8;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerModel;

#[derive(Resource, Default)]
pub struct PlayerVelocity(pub Vec3);

#[derive(Resource)]
pub struct PlayerAnimations {
    pub idle: AnimationNodeIndex,
    pub walking: AnimationNodeIndex,
    pub jumping: AnimationNodeIndex,
    pub crouch_idle: AnimationNodeIndex,
    pub crouch_walking: AnimationNodeIndex,
    pub running: AnimationNodeIndex,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Component, PartialEq, Debug, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    Running,
    Jumping,
    CrouchIdle,
    CrouchWalking,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut graph = AnimationGraph::new();

    let idle = graph.add_clip(
        asset_server.load("models/player.glb#Animation0"),
        1.0,
        graph.root,
    );
    let jumping = graph.add_clip(
        asset_server.load("models/player.glb#Animation1"),
        1.0,
        graph.root,
    );
    let walking = graph.add_clip(
        asset_server.load("models/player.glb#Animation2"),
        1.0,
        graph.root,
    );
    let crouch_idle = graph.add_clip(
        asset_server.load("models/player.glb#Animation3"),
        1.0,
        graph.root,
    );
    let crouch_walking = graph.add_clip(
        asset_server.load("models/player.glb#Animation4"),
        1.0,
        graph.root,
    );
    let running = graph.add_clip(
        asset_server.load("models/player.glb#Animation5"),
        1.0,
        graph.root,
    );

    let graph_handle = graphs.add(graph);

    commands.insert_resource(PlayerAnimations {
        idle,
        walking,
        jumping,
        crouch_idle,
        crouch_walking,
        running,
        graph: graph_handle.clone(),
    });


    commands
        .spawn((
            Transform::from_xyz(0.0, 10.0, 0.0),
            RigidBody::Kinematic,
            Collider::capsule(PLAYER_RADIUS, PLAYER_HEIGHT),
            Player,
            PlayerState::Idle,
        ))
        .with_child((
            SceneRoot(asset_server.load("models/player.glb#Scene0")),
            Transform::from_xyz(0.0, -(PLAYER_HEIGHT / 2.0 + PLAYER_RADIUS), 0.0),
            PlayerModel,
        ));
}

pub fn setup_player_animation(
    mut commands: Commands,
    animations: Res<PlayerAnimations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()));
        player.play(animations.idle).repeat();
    }
}

pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_velocity: ResMut<PlayerVelocity>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut PlayerState,
        ),
        With<Player>,
    >,
    spatial_query: SpatialQuery,
    camera_query: Query<&CameraAngle, With<Camera3d>>,
    time: Res<Time>,
) {
    let Ok((entity, mut transform, mut state)) = query.single_mut() else {
        return;
    };
    let Ok(angle) = camera_query.single() else {
        return;
    };

    let can_stand = spatial_query
        .cast_shape(
            &Collider::cylinder(PLAYER_RADIUS, 0.0),
            transform.translation,
            Quat::IDENTITY,
            Dir3::Y,                                                   // 上方向
            &ShapeCastConfig::from_max_distance(PLAYER_HEIGHT + 0.2),  // 少し余裕を持たせる
            &SpatialQueryFilter::from_excluded_entities(vec![entity]), // 自分自身を除外
        )
        .is_none();

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    // カメラの向きに合わせて移動方向を回転
    let yaw_rotation = Quat::from_rotation_y(angle.yaw());
    let direction = yaw_rotation * direction.normalize_or_zero();

    // 移動方向にプレイヤーを向かせる
    if direction.length_squared() > 0.01 {
        let target_rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
        transform.rotation = transform.rotation.slerp(target_rotation, 0.2);
    }

    let current_height = if matches!(*state, PlayerState::CrouchIdle | PlayerState::CrouchWalking) {
        PLAYER_CROUCH_HEIGHT
    } else {
        PLAYER_HEIGHT
    };

    let grounded_cast_distance = current_height / 2.0 + PLAYER_RADIUS + 0.05;

    // 足元に地面があるか確認
    let grounded = if player_velocity.0.y > 0.0 {
        false // 上昇中は絶対に地面にいない
    } else {
        spatial_query
            .cast_shape(
                &Collider::cylinder(PLAYER_RADIUS * 0.8, 0.0),
                transform.translation, // レイの開始点 (プレイヤーの位置)
                Quat::IDENTITY,
                Dir3::NEG_Y, // 下方向
                &ShapeCastConfig::from_max_distance(grounded_cast_distance),
                &SpatialQueryFilter::from_excluded_entities(vec![entity]), // 自分自身を除外
            )
            .is_some()

    };


    let crouching = keyboard.pressed(KeyCode::ControlLeft) && grounded;
    let has_input = direction.length_squared() > 0.0;
    //let is_moving = player_velocity.0.x.abs() > 0.0 || player_velocity.0.z.abs() > 0.0;

    // PlayerStateを更新
    let next_state = if !grounded {
        PlayerState::Jumping
    } else if crouching {
        if has_input {
            PlayerState::CrouchWalking
        } else {
            PlayerState::CrouchIdle
        }
    } else if matches!(*state, PlayerState::CrouchWalking | PlayerState::CrouchIdle) && !can_stand {
        if has_input {
            PlayerState::CrouchWalking
        } else {
            PlayerState::CrouchIdle
        }
    } else if keyboard.pressed(KeyCode::ShiftLeft) && has_input {
        PlayerState::Running
    } else if has_input {
        PlayerState::Walking
    } else {
        PlayerState::Idle
    };

    if *state != next_state {
        *state = next_state;
    }

    //println!("grounded: {}, direction: {:?}, velocity: {:?}", grounded, direction, player_velocity.0);
    println!("grounded: {}, vy: {}", grounded, player_velocity.0.y);

    let speed = match *state {
        PlayerState::Running => PLAYER_DASH_SPEED,
        PlayerState::CrouchIdle | PlayerState::CrouchWalking => PLAYER_CROUCH_SPEED,
        _ => PLAYER_SPEED,
    };

    let dt = time.delta_secs();

    // 水平方向の速度を設定
    if grounded {
        player_velocity.0.x = direction.x * speed;
        player_velocity.0.z = direction.z * speed;
    } else {
        player_velocity.0.x *=0.99;
        player_velocity.0.z *=0.99;
    }

    // 重力
    if grounded && player_velocity.0.y < 0.0 {
        player_velocity.0.y = 0.0;
    } else {
        player_velocity.0.y += GRAVITY * dt;
    }

    // jump
    if keyboard.just_pressed(KeyCode::Space) 
        && grounded
        && !matches! (*state, PlayerState::CrouchIdle | PlayerState::CrouchWalking) {
        player_velocity.0.y = JUMP_VELOCITY;

    }

    let delta = player_velocity.0 * dt;

    let cast_collider = if matches!(*state, PlayerState::CrouchIdle | PlayerState::CrouchWalking) {
        Collider::capsule(PLAYER_RADIUS, PLAYER_CROUCH_HEIGHT)
    } else {
        Collider::capsule(PLAYER_RADIUS, PLAYER_HEIGHT)
    };

    // Y方向(重量九・ジャンプ)の衝突解決
    let vertical_delta = Vec3::new(0.0, delta.y, 0.0);
    let vertical_dir = if delta.y >= 0.0 { Dir3::Y } else { Dir3::NEG_Y };
    let hit_y = spatial_query.cast_shape(
        &cast_collider,
        transform.translation + Vec3::new(0.05, 0.0, 0.05),
        Quat::IDENTITY,
        vertical_dir,
        &ShapeCastConfig::from_max_distance(delta.y.abs()),
        &SpatialQueryFilter::from_excluded_entities(vec![entity]),
    );
    let vertical_move = if let Some(hit) = hit_y {
        
        if vertical_dir == Dir3::NEG_Y {
            player_velocity.0.y = 0.0;
        } 
            // 頭が天井にぶつかった場合も落下速度をリセット
        vertical_dir.as_vec3() * (hit.distance - 0.01).max(0.0)
    } else {
        vertical_delta
    };

    // X/Z方向 (水平移動) の衝突解決
    let horizontal_delta = Vec3::new(delta.x, 0.0, delta.z);
    let horizontal_move = if horizontal_delta.length_squared() > 0.0 {
        match Dir3::new(horizontal_delta){
            Ok(horizontal_dir) =>{
                let hit_xz = spatial_query.cast_shape(
                    &cast_collider,
                    transform.translation + Vec3::Y*0.1,
                    Quat::IDENTITY,
                    horizontal_dir,
                    &ShapeCastConfig::from_max_distance(horizontal_delta.length()),
                    &SpatialQueryFilter::from_excluded_entities(vec![entity]),
                );
                if let Some(hit) = hit_xz {
                    horizontal_dir.as_vec3() * (hit.distance - 0.01).max(0.0)
                } else {
                    horizontal_delta
                }
            }
            Err(_) => Vec3::ZERO,
        }
    } else {
        Vec3::ZERO
    };

    //println!("move: vertical={:?}, horizontal={:?}", vertical_move, horizontal_move);
    transform.translation += vertical_move + horizontal_move;

}

pub fn update_animation(
    animations: Res<PlayerAnimations>,
    player_query: Query<&PlayerState, With<Player>>,
    mut anim_players: Query<&mut AnimationPlayer>,
    mut current_anim: Local<Option<AnimationNodeIndex>>,
) {
    let Ok(state) = player_query.single() else {
        return;
    };

    let next_anim = match state {
        PlayerState::Idle => animations.idle,
        PlayerState::Walking => animations.walking,
        PlayerState::Jumping => animations.jumping,
        PlayerState::CrouchIdle => animations.crouch_idle,
        PlayerState::CrouchWalking => animations.crouch_walking,
        PlayerState::Running => animations.running,
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
        }
    }
}

pub fn update_player_model_offset(
    player_query: Query<&PlayerState, With<Player>>,
    mut model_query: Query<&mut Transform, (With<PlayerModel>, Without<Player>)>,
) {
    let Ok(state) = player_query.single() else {
        return;
    };

    let target_y = match *state {
        PlayerState::CrouchIdle | PlayerState::CrouchWalking => {
            -(PLAYER_CROUCH_HEIGHT / 2.0 + PLAYER_RADIUS)
        }
        _ => -(PLAYER_HEIGHT / 2.0 + PLAYER_RADIUS),
    };

    for mut transform in &mut model_query {
        transform.translation.y = target_y;
    }
}

pub fn update_player_collider(
    mut commands: Commands,
    mut query: Query<(Entity,  &PlayerState, &mut Transform),(With<Player>, Changed<PlayerState>)>,
    mut was_crouching: Local<bool>,
) {
    let Ok((entity, state, mut transform)) = query.single_mut() else {
        return;
    };
    
    println!("state changed: {:?}", state);

    let is_crouching = matches!(*state, PlayerState::CrouchIdle | PlayerState::CrouchWalking);

    if is_crouching && ! *was_crouching {
        // 立ち->しゃがみ：中心を下げる
        commands.entity(entity).insert(Collider::capsule(PLAYER_RADIUS, PLAYER_CROUCH_HEIGHT));
        transform.translation.y -= (PLAYER_HEIGHT - PLAYER_CROUCH_HEIGHT) / 2.0;
    } else if !is_crouching && *was_crouching {
        // しゃがみ->立ち：中心を上げる
        commands.entity(entity).insert(Collider::capsule(PLAYER_RADIUS, PLAYER_HEIGHT));
        transform.translation.y += (PLAYER_HEIGHT - PLAYER_CROUCH_HEIGHT) / 2.0;
    }

    *was_crouching = is_crouching;

}