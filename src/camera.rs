use bevy::prelude::*;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::window::{CursorGrabMode, CursorOptions};
use avian3d::prelude::*;
use crate::player::Player;

#[derive(Component, PartialEq, Clone, Copy)]
pub enum CameraMode {
    TPS,
    FPS,
}

#[derive(Component)]
pub struct CameraAngle {
    yaw: f32,
    pitch: f32,
    distance: f32,
}

impl Default for CameraAngle{
    fn default() -> Self {
        Self { yaw: 0.0, pitch: 0.3, distance: 10.0 }
    }
}

impl CameraAngle {
    pub fn add_yaw(&mut self, delta: f32) {
        self.yaw -= delta;
    }
    pub fn add_pitch(&mut self, delta: f32) {
        self.pitch = (self.pitch - delta).clamp(-0.5, 1.4);
    }
    pub fn add_distance(&mut self, delta:f32) {
        self.distance = (self.distance - delta).clamp(2.0, 100.0);
    }
    pub fn yaw(&self) -> f32 { self.yaw }
    pub fn pitch(&self) -> f32 { self.pitch }
    pub fn distance(&self) -> f32 { self.distance}
}

    // カメラ
pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(
    (
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraMode::TPS,
        CameraAngle::default(),
    ));
}


pub fn update_camera(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut camera_query: Query<(&mut CameraMode, &mut CameraAngle), With<Camera3d>>,
) {
    let Ok((mut mode, mut angle)) = camera_query.single_mut() else {return; };

    // モード切替
    if keyboard.just_pressed(KeyCode::KeyV) {
        *mode = match *mode {
            CameraMode::TPS => CameraMode::FPS,
            CameraMode::FPS => CameraMode::TPS,
        };
    }

    let sensitivity = 0.003;
    angle.add_yaw(mouse_motion.delta.x * sensitivity);
    angle.add_pitch(mouse_motion.delta.y * sensitivity);

    angle.add_distance(mouse_scroll.delta.y);
}

pub fn camera_follow(
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut camera_query: Query<(&mut Transform,&CameraMode, &CameraAngle), (With<Camera3d>, Without<Player>)>,
    spatial_query: SpatialQuery,
) {
    let Ok((player_entity, player_transform)) = player_query.single() else { return; };
    let Ok((mut camera_transform, mode, angle)) = camera_query.single_mut() else { return; };


    let rotation = Quat::from_rotation_y(angle.yaw())
        * Quat::from_rotation_x(angle.pitch());

    match *mode {
        CameraMode::TPS => {
            let ideal_offset = rotation * Vec3::new(0.0, 5.0, angle.distance());
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

pub fn handle_focus(
    window: Single<&Window>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    if window.focused && mouse_button.just_pressed(MouseButton::Left) {
        cursor_options.grab_mode = CursorGrabMode::Locked;
        cursor_options.visible = false;
    } else if !window.focused {
        cursor_options.grab_mode = CursorGrabMode::None;
        cursor_options.visible = true;
    }
}

