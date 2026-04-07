use bevy::prelude::*;
use avian3d::prelude::*;

mod world;
mod player;

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
        .add_plugins(NoCameraPlayerPlugin)  // PlayerPluginの代わり、カメラは自分でspawnする
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(player::CameraMode::TPS)
        .insert_resource(player::CameraAngle::default()) 
        .add_systems(Startup, (world::setup, player::spawn_player))    
        .add_systems(Update, (
            world::asset_loaded,
            player::switch_camera_mode,
            player::mouse_look,
            player::move_player,
            player::camera_follow,
            close_on_esc,
        ))
        .run();
}

