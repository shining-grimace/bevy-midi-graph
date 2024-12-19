use bevy::{
    asset::load_internal_asset,
    gltf::GltfPlugin,
    pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline},
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderDefVal, ShaderRef,
            SpecializedMeshPipelineError, VertexFormat,
        },
    },
};

const ARRAY_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(49343755002075437171731282533994485280);

const ATTRIBUTE_ATLAS_INDEX_0: MeshVertexAttribute =
    MeshVertexAttribute::new("AtlasIndex0", 63728, VertexFormat::Float32);

const ATTRIBUTE_ATLAS_INDEX_1: MeshVertexAttribute =
    MeshVertexAttribute::new("AtlasIndex1", 63729, VertexFormat::Float32);

const ATTRIBUTE_ATLAS_BLEND: MeshVertexAttribute =
    MeshVertexAttribute::new("AtlasBlend", 63730, VertexFormat::Float32);

pub struct AcornMaterialPlugin;

impl Plugin for AcornMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(
                GltfPlugin::default()
                    .add_custom_vertex_attribute("ATLAS_INDEX_0", ATTRIBUTE_ATLAS_INDEX_0)
                    .add_custom_vertex_attribute("ATLAS_INDEX_1", ATTRIBUTE_ATLAS_INDEX_1)
                    .add_custom_vertex_attribute("ATLAS_BLEND", ATTRIBUTE_ATLAS_BLEND)
            ),
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, ArrayTextureMaterialExtension>>::default(),
        ));
        load_internal_asset!(
            app,
            ARRAY_SHADER_HANDLE,
            "../../assets/acorn/array_texture.wgsl",
            Shader::from_wgsl
        );
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ArrayTextureMaterialExtension {
    #[texture(100, dimension = "2d_array")]
    #[sampler(101)]
    pub array_texture_0: Handle<Image>,

    #[texture(102, dimension = "2d_array")]
    #[sampler(103)]
    pub array_texture_1: Handle<Image>,
}

// https://github.com/splashdust/bevy_voxel_world/blob/main/src/voxel_material.rs
impl MaterialExtension for ArrayTextureMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        ARRAY_SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        ARRAY_SHADER_HANDLE.into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This shader isn't compatible with a prepass
        if descriptor
            .vertex
            .shader_defs
            .contains(&ShaderDefVal::Bool("PREPASS_PIPELINE".into(), true))
        {
            panic!("This shader isn't designed for a prepass");
        }

        let all_attributes = [
            // https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/render/forward_io.wgsl
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            Mesh::ATTRIBUTE_UV_1.at_shader_location(3),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(5),
            // Custom
            ATTRIBUTE_ATLAS_INDEX_0.at_shader_location(8),
            ATTRIBUTE_ATLAS_INDEX_1.at_shader_location(9),
            ATTRIBUTE_ATLAS_BLEND.at_shader_location(10),
        ];
        let vertex_layout = layout.0.get_layout(&all_attributes)?;
        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}
