use serde::{Deserialize, Serialize};
pub mod bevy;

///The save file data for the voxel world. voxel values correspond to an index in voxel_materials.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct WorldSaveData {
    pub voxel_materials: Vec<VoxelMaterial>,
    pub voxels: VoxelTree,
    pub seed: u64,
    pub entities: Vec<Entity>,
}

/// The data for a wasm file
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct VoxelMaterial {
    pub name: String,
    pub wasm: Vec<u8>
}

/// The data for an entity
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Entity {
    pub position: Vec3,
    pub voxels: VoxelTree
}

///A 3D vector
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// The voxel data. This is stored as a sparse voxel octree. 
/// Voxel values are u16s and correspond to an index in voxel_materials. 
/// The material function will be called, returning the actual color for that voxel.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct VoxelTree {
    pub data: Vec<u8>
}