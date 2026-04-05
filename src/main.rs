use bevy::prelude::*;
use bevy::core_pipeline::Skybox;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
use bevy_flycam::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)  // PlayerPluginの代わり、カメラは自分でspawnする
        .add_systems(Startup, setup)
        .add_systems(Update, asset_loaded)
        .run();
}

#[derive(Resource)]
struct Skyboxhandle {
    image: Handle<Image>,
    is_loaded: bool,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let skybox_handle: Handle<Image> = asset_server.load("textures/Ryfjallet_cubemap.png");
    commands.insert_resource(Skyboxhandle {
        image: skybox_handle.clone(),
        is_loaded: false,
    });

    // カメラ (FlyCamコンポーネントを付けることでWASD操作可能に)
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
    ));

    //　太陽光
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0,8.0,4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(50.0, 50.0)))),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.8, 0.5))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 箱
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.8, 0.5))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // 箱を複数配置
    let positions = [
        (0.0, -2.0, 2.0, Color::srgb(0.8, 0.6, 0.4)),// (x, z, 高さ) ベージュ
        (3.0, -4.0, 1.0, Color::srgb(0.6, 0.6, 0.8)), // 青っぽい
        (-3.0, -6.0, 4.0, Color::srgb(0.8, 0.4, 0.4)), // 赤っぽい
        (5.0, -8.0, 1.5, Color::srgb(0.8, 0.9, 0.6)), // yellow
        (-5.0, -5.0, 3.0, Color::srgb( 0.5, 0.7, 0.5)), // green

    ];

    for (x, z, height, color) in positions {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, height, 1.0))),
            MeshMaterial3d(materials.add(color)),
            Transform::from_xyz(x, height / 2.0,  z),
        ));
    }


}

fn asset_loaded(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut skybox_res: ResMut<Skyboxhandle>,
    camera_query: Query<Entity, With<Camera3d>>,
) {
    if skybox_res.is_loaded { return;}
    if !asset_server.load_state(&skybox_res.image).is_loaded() { return;}


    let image = images.get_mut(&skybox_res.image).unwrap();
    if image.texture_descriptor.array_layer_count() == 1 {
        let layers = image.height() / image.width();
        let _= image.reinterpret_stacked_2d_as_array(layers);
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }

    if let Ok(entity) = camera_query.single(){
        commands.entity(entity).insert(Skybox {
            image: skybox_res.image.clone(),
            brightness: 1000.0,
            ..default()
        });

    }

    skybox_res.is_loaded = true;

}