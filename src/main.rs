use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};
use avian3d::prelude::*;


mod world;
mod player;
mod camera;

fn close_on_esc(
    keyboad: Res<ButtonInput<KeyCode>>,
    mut cursor_options: Single<&mut CursorOptions>,
    mut exit: MessageWriter<AppExit>,
) {
    if keyboad.just_pressed(KeyCode::Escape) {
        cursor_options.grab_mode = CursorGrabMode::None;
        cursor_options.visible = true;
        exit.write(AppExit::Success);
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, (
            world::setup,
            player::spawn_player,
            camera::spawn_camera,
        ))    
        .add_systems(Update, (
            world::asset_loaded,
            camera::update_camera,
            player::move_player,
            camera::camera_follow,
        ).chain())
        .add_systems(Update, camera::handle_focus)
        .add_systems(Update, close_on_esc)
        .run();
}

