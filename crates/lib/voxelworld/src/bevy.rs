use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use wasmtime::{Engine, Module, Instance, Store, Func};

use crate::WorldSaveData;

pub struct VoxelWorldPlugin;

impl Plugin for VoxelWorldPlugin {
    fn build(&self, app: &mut App) {

    }
}

// Add this component to the world for every voxel world you want to exist.
#[derive(Component)]
pub struct VoxelWorld {
    pub save_data: WorldSaveData,
    pub wasm_engine: Engine,
    pub wasm_store: Store<()>,
    pub wasm_material_instances: Vec<Instance>,
}
impl VoxelWorld {
    pub fn new(save_data: WorldSaveData) -> Self {
        let wasm_engine = Engine::default();
        let mut wasm_store = Store::new(&wasm_engine, ());
        let mut wasm_material_instances = Vec::new();

        for voxel_material in &save_data.voxel_materials{
            let module = Module::new(&wasm_engine, &voxel_material.wasm).unwrap();
            let instance = Instance::new(&mut wasm_store, &module, &[]).unwrap();
            wasm_material_instances.push(instance);
        }
        
        Self {
            save_data,
            wasm_engine,
            wasm_store,
            wasm_material_instances,
        }
    }
    pub fn get_voxel_color(&self, material: u16, x: u64, y: u64, z: u64, lod: u8) -> Option<Color> {
        let instance = self.wasm_material_instances[material as usize];
        let mut wasm_store = Store::new(&self.wasm_engine, ());
        let func = instance.get_typed_func::<(u64, u64, u64, u64, u64), (u32, f32, f32, f32)>(&mut wasm_store, "get_color").unwrap();
        let (voxel_type, r, g, b) = func.call(&mut wasm_store, (self.save_data.seed, x, y, z, lod as u64)).unwrap();

        match voxel_type {
            0 => None,
            1 => Some(Color::rgb(r, g, b)),
            _ => panic!("Invalid voxel type returned from wasm function"),
        }

    }
}

/// Add this component to an entity to make it a point of interest for the voxel world. (Must have a Transform component) 
/// This will use maximum level of detail in those areas of the world. 
/// Attach this to any cameras that need to view the world.
#[derive(Component)]
pub struct VoxelPointOfInterest {
    pub voxel_world_entity: Entity,
    pub range: f32,
}

/// This bevy system will render all voxel worlds and their entities. 
/// Level of detail in different areas is based on locations of entities with VoxelPointOfInterest.
pub fn render_voxel_worlds_system(points_of_interest_query: Query<(&Transform, &VoxelPointOfInterest)>) {

    
    //for each point of interest, calculate desired list of chunks and their sizes
    let mut chunks = Vec::<Chunk>::new();
    points_of_interest_query.iter().for_each(|(transform, point_of_interest)| {
        chunks.push(Chunk { 
            x: transform.local_x(), 
            y: transform.local_y(), 
            z: transform.local_z(), 
            lod: 0 
        });

        // need: 
        // max lod
        // chunk size (voxels per chunk)
        // voxel size (voxels per metre)

    });

    //remove duplicates from list

    //remove chunks already existing

    //calculate meshes for each chunk in list
    
    //replace existing meshes with new meshes

}

pub struct Chunk {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub lod: u8,
}