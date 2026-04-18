use avian3d::prelude::*;
use bevy::core_pipeline::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
use rand::Rng;

const WORLD_ILLUMINANCE: f32 = 10000.0;
const WORLD_BRIGHTNESS: f32 = 1000.0;
const WORLD_HALF_EXTENT: f32 = 300.0;

const WORLD_BUILDING_SIZE_MIN: f32 = 3.0;
const WORLD_BUILDING_SIZE_MAX: f32 = 8.0;
const WORLD_BUILDING_HEIGHT_MIN: f32 = 4.0;
const WORLD_BUILDING_HEIGHT_MAX: f32 = 16.0;
const WORLD_ROAD_WIDTH: f32 = 3.0;

pub struct CityConfig {
    pub grid_count: usize,
    pub origin: Vec3,
    pub building_height_max: f32,
    pub road_width: f32,
}

impl CityConfig {
    pub fn new(grid_count: usize, origin: Vec3) -> Self {
        Self {
            grid_count,
            origin,
            building_height_max: WORLD_BUILDING_HEIGHT_MAX,
            road_width: WORLD_ROAD_WIDTH,
        }
    }
}

#[derive(Resource)]
pub struct SkyboxHandle {
    pub image: Handle<Image>,
    pub is_loaded: bool,
}

/// Spawn ground, building, lights, and skybox texture handle
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let skybox_handle: Handle<Image> = asset_server.load("textures/Ryfjallet_cubemap.png");
    commands.insert_resource(SkyboxHandle {
        image: skybox_handle.clone(),
        is_loaded: false,
    });

    // Sunlight
    commands.spawn((
        DirectionalLight {
            illuminance: WORLD_ILLUMINANCE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(WORLD_HALF_EXTENT, WORLD_HALF_EXTENT)))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(WORLD_HALF_EXTENT * 2.0, 0.0, WORLD_HALF_EXTENT * 2.0),
    ));

    // City grid
    let cities = vec![
        CityConfig::new(15, Vec3::new(0.0, 0.0, 0.0)),
        CityConfig::new(5, Vec3::new(200.0, 0.0, 0.0)),
        CityConfig::new(8, Vec3::new(0.0, 0.0, 200.0)),
    ];

    for city in &cities {
        let grid_total = city.grid_count as f32 * (WORLD_BUILDING_SIZE_MAX + city.road_width);
        let offset = grid_total / 2.0;
        let mut rng = rand::thread_rng();

        for x in 0..city.grid_count{
            for z in 0..city.grid_count {

                let height = rng.gen_range(WORLD_BUILDING_HEIGHT_MIN..city.building_height_max);
                let size_x: f32 = rng.gen_range(WORLD_BUILDING_SIZE_MIN..WORLD_BUILDING_SIZE_MAX);
                let size_z: f32 = rng.gen_range(WORLD_BUILDING_SIZE_MIN..WORLD_BUILDING_SIZE_MAX);

                let colors = [
                    Color::srgb(0.7, 0.7, 0.7),  // グレー
                    Color::srgb(0.8, 0.7, 0.5),  // ベージュ
                    Color::srgb(0.5, 0.6, 0.8),  // 青系
                    Color::srgb(0.6, 0.6, 0.6),  // 濃いグレー
                    Color::srgb(0.8, 0.8, 0.7),  // クリーム
                ];
                let color = colors[rng.gen_range(0..colors.len())]; 
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(size_x,height,size_z))),
                    MeshMaterial3d(materials.add(color)),
                    Transform::from_xyz(
                        x as f32 * (WORLD_BUILDING_SIZE_MAX+ city.road_width) - offset + city.origin.x,
                        height / 2.0,
                        z as f32 * (WORLD_BUILDING_SIZE_MAX + city.road_width) - offset + city.origin.z),
                    RigidBody::Static,
                    Collider::cuboid(size_x, height, size_z),
                ));
            }
        }
    }
}

/// Attach skybox to camera once the cubemap texture finishes loading.
pub fn asset_loaded(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut skybox_res: ResMut<SkyboxHandle>,
    camera_query: Query<Entity, With<Camera3d>>,
) {
    if skybox_res.is_loaded {
        return;
    }
    if !asset_server.load_state(&skybox_res.image).is_loaded() {
        return;
    } 

    let Some(image) = images.get_mut(&skybox_res.image) else { return; };
    if image.texture_descriptor.array_layer_count() == 1 {
        let layers = image.height() / image.width();
        let _ = image.reinterpret_stacked_2d_as_array(layers);
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }

    if let Ok(entity) = camera_query.single() {
        commands.entity(entity).insert(Skybox {
            image: skybox_res.image.clone(),
            brightness: WORLD_BRIGHTNESS,
            ..default()
        });
    }
    skybox_res.is_loaded = true;
}
