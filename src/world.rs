use avian3d::prelude::*;
use bevy::core_pipeline::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
use rand::Rng;

const WORLD_ILLUMINANCE: f32 = 10000.0;
const WORLD_BRIGHTNESS: f32 = 1000.0;
const WORLD_HALF_EXTENT: f32 = 300.0;

const BUILDING_FLOORS_MIN: usize = 1;
const BUILDING_FLOORS_MAX: usize = 5;

// Wall geometry (measured from gltf bounding box)
// Pivot: X=center, Y=bottom, Z=near front face (0.31 behind, 0.09 in front)
const WALL_SIZE: Vec3 = Vec3::new(2.00, 3.12, 0.41);

#[derive(Copy, Clone)]
enum RoofSize {
    R4x4,
    R4x6,
    R4x8,
    R6x4,
    R6x6,
    R6x8,
    R6x10,
    R6x12,
    R6x14,
    R8x8,
    R8x10,
    R8x12,
    R8x14,
}

impl RoofSize {
    /// Returns (width_walls, depth_walls) based on roof name (X/2 x Y/2 walls)
    fn wall_counts(&self) -> (usize, usize) {
        match self {
            Self::R4x4 => (2, 2),
            Self::R4x6 => (2, 3),
            Self::R4x8 => (2, 4),
            Self::R6x4 => (3, 2),
            Self::R6x6 => (3, 3),
            Self::R6x8 => (3, 4),
            Self::R6x10 => (3, 5),
            Self::R6x12 => (3, 6),
            Self::R6x14 => (3, 7),
            Self::R8x8 => (4, 4),
            Self::R8x10 => (4, 5),
            Self::R8x12 => (4, 6),
            Self::R8x14 => (4, 7),
        }
    }

    fn asset_path(&self) -> &'static str {
        match self {
            Self::R4x4 => "medieval/Roof_RoundTiles_4x4.gltf#Scene0",
            Self::R4x6 => "medieval/Roof_RoundTiles_4x6.gltf#Scene0",
            Self::R4x8 => "medieval/Roof_RoundTiles_4x8.gltf#Scene0",
            Self::R6x4 => "medieval/Roof_RoundTiles_6x4.gltf#Scene0",
            Self::R6x6 => "medieval/Roof_RoundTiles_6x6.gltf#Scene0",
            Self::R6x8 => "medieval/Roof_RoundTiles_6x8.gltf#Scene0",
            Self::R6x10 => "medieval/Roof_RoundTiles_6x10.gltf#Scene0",
            Self::R6x12 => "medieval/Roof_RoundTiles_6x12.gltf#Scene0",
            Self::R6x14 => "medieval/Roof_RoundTiles_6x14.gltf#Scene0",
            Self::R8x8 => "medieval/Roof_RoundTiles_8x8.gltf#Scene0",
            Self::R8x10 => "medieval/Roof_RoundTiles_8x10.gltf#Scene0",
            Self::R8x12 => "medieval/Roof_RoundTiles_8x12.gltf#Scene0",
            Self::R8x14 => "medieval/Roof_RoundTiles_8x14.gltf#Scene0",            
        }
    }
}

struct BuildingPlan {
    roof: RoofSize,
    floor_count: usize,
}

impl BuildingPlan {
    /// Returns (width, depth) of the building footprint in meters.
    fn footprint(&self) -> (f32, f32) {
        let (w,d) = self.roof.wall_counts();
        (w as f32 * WALL_SIZE.x, d as f32 * WALL_SIZE.x)
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
        Collider::cuboid(WORLD_HALF_EXTENT * 2.0, 0.1, WORLD_HALF_EXTENT * 2.0),
    ));

    // generate one column of buildings
    let mut rng = rand::thread_rng();

    const ROAD_DEPTH: f32 = 4.0;    // width between house and house in a same column
    const ROAD_WIDTH: f32 = 6.0;    // Gap between columns （X direction, wide enough for a carriage)
    const COLUMN_COUNT: usize = 3; 
    const BUILDINGS_PER_COLUMN: usize = 5;
    
    let mut current_x = 0.0;

    for _ in 0..COLUMN_COUNT {
        // Generate houses in this column
        let column: Vec<BuildingPlan> = (0..BUILDINGS_PER_COLUMN)
            .map(|_| random_building_plan(&mut rng))
            .collect();

        // calculate the max width in this column ( X direction width = footprint.0)
        let column_width = column.iter()
            .map(|p| p.footprint().0)
            .fold(0.0f32, f32::max);

        // Align houses in this column
        let mut current_z = 0.0;
        for plan in &column {
            let ( w, d ) = plan.footprint();
            // 
            let origin = Vec3::new(
                current_x + (column_width -w ) / 2.0,
                0.0,
                -current_z,
            );
            spawn_building(&mut commands, &asset_server, origin, plan.roof, plan.floor_count);
            current_z += d + ROAD_DEPTH;
        }

        // to Next Column
        current_x += column_width + ROAD_WIDTH;

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

/// Draw debug grid and axes (dev only)
pub fn draw_debug_gizmos(mut gizmos: Gizmos) {
    // XZ Plane (ground) - white
    gizmos.grid(
        Isometry3d::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        UVec2::new(20, 20),
        Vec2::splat(2.0),
        Color::srgba(1.0, 1.0, 1.0, 0.3),
    );

    // XY Plane 
    gizmos.grid(
        Isometry3d::IDENTITY,
        UVec2::new(20, 20),
        Vec2::splat(2.0),
        Color::srgba(0.0, 0.5, 1.0, 0.2),
    );
    
    // YZ Plane
    gizmos.grid(
        Isometry3d::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        UVec2::new(20, 20),
        Vec2::splat(2.0),
        Color::srgba(0.0, 1.0, 0.5, 0.2),
    );

    // Axis
    gizmos.arrow(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0), Color::srgb(1.0, 0.0, 0.0));
    gizmos.arrow(Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0), Color::srgb(0.0, 1.0, 0.0));
    gizmos.arrow(Vec3::ZERO, Vec3::new(0.0, 0.0, 10.0), Color::srgb(0.0, 0.0, 1.0));
}

fn spawn_building(
    commands: &mut Commands,
    asset_server: &AssetServer,
    origin: Vec3,
    roof: RoofSize,
    floor_count: usize,
) {
    let (width_count, depth_count) = roof.wall_counts();

    // Building footprint dimensions
    let total_width = width_count as f32 * WALL_SIZE.x;
    let total_depth = depth_count as f32 * WALL_SIZE.x;

    // Center of the building footprint (used as the anchor for the roof)
    let center_x = origin.x + total_width / 2.0;
    let center_z = origin.z - total_depth / 2.0;

    for floor in 0..floor_count {
        let y = origin.y + floor as f32 * WALL_SIZE.y;

        for col in 0..width_count {
            let x = origin.x + (col as f32 + 0.5) * WALL_SIZE.x;

            // Front wall (no rotation)
            spawn_wall (
                commands,
                asset_server,
                Transform::from_xyz(x, y, origin.z),
            );

            // Back wall (rotated 180 degree)
            spawn_wall(
                commands,
                asset_server,
                Transform::from_xyz(x, y, origin.z -total_depth)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            );
        }

        for row in 0..depth_count {
            let z = origin.z - (row as f32 + 0.5)* WALL_SIZE.x;
            // Left wall (rotated -90 degrees)
            spawn_wall(
                commands,
                asset_server,
                Transform::from_xyz(origin.x, y, z)
                    .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
            );

            // Right wall (rotated +90 degrees)
            spawn_wall (
                commands,
                asset_server,
                Transform::from_xyz(origin.x + total_width, y, z)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            );
        }
    }

    // Roof: pivot is at the geometric center, so just place at building center
    let roof_y = origin.y + floor_count as f32 * WALL_SIZE.y;
    commands.spawn((
        SceneRoot(asset_server.load(roof.asset_path())),
        Transform::from_xyz(center_x, roof_y, center_z),
    ));

    // Single building-level collider covering the whole footprint.
    // Walls are visual-only; this static box prevents the player from
    // walking through the building.
    let building_height = floor_count as f32 * WALL_SIZE.y;
    let building_center = Vec3::new(
        origin.x + total_width / 2.0,
        origin.y + building_height / 2.0,
        origin.z - total_depth / 2.0,
    );   

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(total_width, building_height, total_depth),
        Transform::from_translation(building_center),
    ));

}


/// Spawn a single wall (visual mesh only).
/// Collision is handled at the building level in `spawn_building`.
fn spawn_wall(
    commands: &mut Commands,
    asset_server: &AssetServer,
    transform: Transform,
) {
    const WALL_ASSET: &str = "medieval/Wall_Plaster_Straight.gltf#Scene0";

    commands
        .spawn((
            SceneRoot(asset_server.load(WALL_ASSET)),
            transform,
        ));
}


fn random_building_plan(rng: &mut impl Rng) -> BuildingPlan {
    use RoofSize::*;
    let roofs = [
        R4x4, R4x6, R4x8,
        R6x4, R6x6, R6x8, R6x10, R6x10, R6x12, R6x14,
        R8x8, R8x10, R8x12, R8x14,
    ];
    let roof = roofs[rng.gen_range(0..roofs.len())];
    let floor_count = rng.gen_range(BUILDING_FLOORS_MIN..=BUILDING_FLOORS_MAX);
    BuildingPlan { roof, floor_count }
}