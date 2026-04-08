use bevy::prelude::*;
use avian3d::prelude::*;

mod world;
mod player;
mod camera;

fn close_on_esc(
    keyboad: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keyboad.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, (world::setup, player::spawn_player, camera::spawn_camera))    
        .add_systems(Update, (
            world::asset_loaded,
            camera::update_camera,
            camera::camera_follow,
            player::move_player,
            close_on_esc,
        ))
        .run();
}

