use bevy::prelude::*;
use bevy_flycam::prelude::*;
use avian3d::prelude::*;

mod world;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)  // PlayerPluginの代わり、カメラは自分でspawnする
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, (world::setup, player::spawn_player))
        .add_systems(Update, (world::asset_loaded, player::move_player))
        .run();
}
