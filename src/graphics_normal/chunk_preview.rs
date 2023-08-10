/* 
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use voxels::data::voxel_octree::VoxelMode;
use crate::graphics::{ChunkPreviewGraphics, GraphicsResource};
use crate::data::GameResource;
use super::chunks::{CustomMaterial, VOXEL_COLOR};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(spawn);
  }
}

fn spawn(
  mut local_res: ResMut<LocalResource>,
  graphics_res: Res<GraphicsResource>,

  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut game_res: ResMut<GameResource>,
  mut chunk_previews: Query<&ChunkPreview>,

  mut custom_materials: ResMut<Assets<CustomMaterial>>,

  graphics: Query<(Entity, &ChunkPreviewGraphics)>,
) {
  for (graphics_entity, graphics) in &graphics {
    if entity == graphics.parent {
      commands.entity(graphics_entity).despawn_recursive();
    }
  }


  let preview = chunk_previews.get_mut(local_res.preview_entity).unwrap();
  let chunk = local_res.chunk_op.take().unwrap();
  let data = chunk.octree.compute_mesh(
    VoxelMode::SurfaceNets, 
    &mut game_res.chunk_manager.voxel_reuse.clone(),
    &game_res.colors,
  );

  if data.indices.len() > 0 { // Temporary, should be removed once the ChunkMode detection is working
    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    render_mesh.insert_attribute(VOXEL_COLOR, data.colors.clone());

    let mesh_handle = meshes.add(render_mesh);
    let material_handle = custom_materials.add(CustomMaterial {
      base_color: Color::rgb(1.0, 1.0, 1.0),
    });

    let chunk_size = (chunk.octree.get_size() / 2) as f32;
    let p = &preview.new;
    let adj = [p[0] as f32, p[1] as f32, p[2] as f32];
    let coord_f32 = [adj[0] - chunk_size, adj[1] - chunk_size, adj[2] - chunk_size];

    let mut visibility = Visibility::Visible;
    if !graphics_res.show_preview {
      visibility = Visibility::Hidden;
    }

    commands
      .spawn(MaterialMeshBundle {
        visibility: visibility,
        mesh: mesh_handle,
        material: material_handle,
        transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
          // .with_scale(Vec3::new(0.99, 0.999, 0.99 )),
        ..default()
      })
      .insert(ChunkPreviewGraphics { parent: local_res.preview_entity });
  }

}
 */

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use voxels::data::voxel_octree::VoxelMode;
use crate::components::chunk_edit::{ChunkEdit, EditState};
use crate::graphics::ChunkPreviewGraphics;
use crate::data::GameResource;

use super::chunks::{CustomMaterial, VOXEL_COLOR};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update_add.run_if(add_state))
      .add_system(update_remove.run_if(remove_state));
  }
}

fn add_state(state: Res<State<EditState>>,) -> bool {
  state.0 == EditState::AddNormal ||
  state.0 == EditState::AddSnap
}

fn remove_state(state: Res<State<EditState>>,) -> bool {
  state.0 == EditState::RemoveNormal ||
  state.0 == EditState::RemoveSnap
}


fn update_add(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  game_res: Res<GameResource>,
  edits: Query<(Entity, &ChunkEdit), Changed<ChunkEdit>>,
  graphics: Query<(Entity, &ChunkPreviewGraphics)>,

  mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
  for (entity, edit) in &edits {
    for (graphics_entity, graphics) in &graphics {
      if entity == graphics.parent {
        commands.entity(graphics_entity).despawn_recursive();
      }
    }

    if edit.chunk.is_none() {
      continue;
    }

    let chunk = edit.chunk.clone().unwrap();

    let data = chunk.octree.compute_mesh(
      VoxelMode::SurfaceNets, 
      &mut game_res.chunk_manager.voxel_reuse.clone(),
      &game_res.colors,
    );

    if data.indices.len() > 0 { // Temporary, should be removed once the ChunkMode detection is working
      let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
      render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

      render_mesh.insert_attribute(VOXEL_COLOR, data.colors.clone());

      let mesh_handle = meshes.add(render_mesh);
      let material_handle = custom_materials.add(CustomMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
      });


      let chunk_size = (chunk.octree.get_size() / 2) as f32;
      let p = &edit.position.unwrap();
      let adj = [p.x as f32, p.y as f32, p.z as f32];
      let coord_f32 = [adj[0] - chunk_size, adj[1] - chunk_size, adj[2] - chunk_size];

      commands
        .spawn(MaterialMeshBundle {
          // visibility: visibility,
          mesh: mesh_handle,
          material: material_handle,
          transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
          ..default()
        })
        .insert(ChunkPreviewGraphics { parent: entity });
    }
  }
}

fn update_remove() {

}

