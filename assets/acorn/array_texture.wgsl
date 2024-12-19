
// Override base PBR shaders, added attribute for atlas index. Supporting Bevy 0.15.
//
// This currently doesn't support deferred rendering, or a prepass, or meshlets, or order-independent transparency.
// To integrate those features, the original Bevy sources need to be copied in here again and updated as needed.
//
// Forward rendering base code:
// - IO structs: https://github.com/bevyengine/bevy/blob/release-0.15.0/crates/bevy_pbr/src/render/forward_io.wgsl
// - Vertex shader: https://github.com/bevyengine/bevy/blob/release-0.15.0/crates/bevy_pbr/src/render/mesh.wgsl
// - Fragment shader: https://github.com/bevyengine/bevy/blob/release-0.15.0/crates/bevy_pbr/src/render/pbr.wgsl
//
// Prepass base code:
// - IO structs: https://github.com/bevyengine/bevy/blob/release-0.15.0/crates/bevy_pbr/src/prepass/prepass_io.wgsl
// - Shaders: https://github.com/bevyengine/bevy/blob/release-0.15.0/crates/bevy_pbr/src/prepass/prepass.wgsl

// Merged imports from default vertex and fragment shaders, plus the needs of custom shader code
#import bevy_pbr::{
    mesh_bindings::mesh,
    mesh_functions,
    skinning,
    morph::morph,
    forward_io::{Vertex, VertexOutput, FragmentOutput},
    view_transformations::position_world_to_clip,
    pbr_types,
    pbr_functions::alpha_discard,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions,
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
}

struct CustomVertex {
    @builtin(instance_index) instance_index: u32,
#ifdef VERTEX_POSITIONS
    @location(0) position: vec3<f32>,
#endif
#ifdef VERTEX_NORMALS
    @location(1) normal: vec3<f32>,
#endif
#ifdef VERTEX_UVS_A
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_UVS_B
    @location(3) uv_b: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(4) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(5) color: vec4<f32>,
#endif
#ifdef SKINNED
    @location(6) joint_indices: vec4<u32>,
    @location(7) joint_weights: vec4<f32>,
#endif
#ifdef MORPH_TARGETS
    @builtin(vertex_index) index: u32,
#endif

    // Custom: texture atlas index attributes
    @location(8) atlas_index_0: f32,
    @location(9) atlas_index_1: f32,
    @location(10) atlas_blend: f32
};

struct CustomVertexOutput {
    // This is `clip position` when the struct is used as a vertex stage output
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
#ifdef VERTEX_UVS_A
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_UVS_B
    @location(3) uv_b: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(4) world_tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(5) color: vec4<f32>,
#endif
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    @location(6) @interpolate(flat) instance_index: u32,
#endif
#ifdef VISIBILITY_RANGE_DITHER
    @location(7) @interpolate(flat) visibility_range_dither: i32,
#endif

    // Custom: texture atlas index attributes
    @location(8) atlas_index_0: i32,
    @location(9) atlas_index_1: i32,
    @location(10) atlas_blend: f32
}

fn pbr_vertex_from_custom_vertex(in: CustomVertexOutput) -> VertexOutput {
    var out: VertexOutput;

    out.position = in.position;
    out.world_position = in.world_position;
    out.world_normal = in.world_normal;
#ifdef VERTEX_UVS_A
    out.uv = in.uv;
#endif
#ifdef VERTEX_UVS_B
    out.uv_b = in.uv_b;
#endif
#ifdef VERTEX_TANGENTS
    out.world_tangent = in.world_tangent;
#endif
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    out.instance_index = in.instance_index;
#endif
#ifdef VISIBILITY_RANGE_DITHER
    out.visibility_range_dither = in.visibility_range_dither;
#endif
    return out;
}

// Custom: sampler for array texture
@group(2) @binding(100) var custom_array_texture_0: texture_2d_array<f32>;
@group(2) @binding(101) var custom_array_texture_sampler_0: sampler;
@group(2) @binding(102) var custom_array_texture_1: texture_2d_array<f32>;
@group(2) @binding(103) var custom_array_texture_sampler_1: sampler;

#ifdef MORPH_TARGETS
fn morph_vertex(vertex_in: CustomVertex) -> CustomVertex {
    var vertex = vertex_in;
    let first_vertex = mesh[vertex.instance_index].first_vertex_index;
    let vertex_index = vertex.index - first_vertex;

    let weight_count = bevy_pbr::morph::layer_count();
    for (var i: u32 = 0u; i < weight_count; i ++) {
        let weight = bevy_pbr::morph::weight_at(i);
        if weight == 0.0 {
            continue;
        }
        vertex.position += weight * morph(vertex_index, bevy_pbr::morph::position_offset, i);
#ifdef VERTEX_NORMALS
        vertex.normal += weight * morph(vertex_index, bevy_pbr::morph::normal_offset, i);
#endif
#ifdef VERTEX_TANGENTS
        vertex.tangent += vec4(weight * morph(vertex_index, bevy_pbr::morph::tangent_offset, i), 0.0);
#endif
    }
    return vertex;
}
#endif

@vertex
fn vertex(vertex_no_morph: CustomVertex) -> CustomVertexOutput {
    var out: CustomVertexOutput;

#ifdef MORPH_TARGETS
    var vertex = morph_vertex(vertex_no_morph);
#else
    var vertex = vertex_no_morph;
#endif

#ifdef SKINNED
    var world_from_local = skinning::skin_model(vertex.joint_indices, vertex.joint_weights);
#else
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416 .
    var world_from_local = mesh_functions::get_world_from_local(vertex_no_morph.instance_index);
#endif

#ifdef VERTEX_NORMALS
#ifdef SKINNED
    out.world_normal = skinning::skin_normals(world_from_local, vertex.normal);
#else
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        vertex_no_morph.instance_index
    );
#endif
#endif

#ifdef VERTEX_POSITIONS
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
#endif

#ifdef VERTEX_UVS_A
    out.uv = vertex.uv;
#endif
#ifdef VERTEX_UVS_B
    out.uv_b = vertex.uv_b;
#endif

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        world_from_local,
        vertex.tangent,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        vertex_no_morph.instance_index
    );
#endif

#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    out.instance_index = vertex_no_morph.instance_index;
#endif

#ifdef VISIBILITY_RANGE_DITHER
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
        vertex_no_morph.instance_index, world_from_local[3]);
#endif

    // Custom: pass through texture atlas attributes
    out.atlas_index_0 = i32(vertex.atlas_index_0);
    out.atlas_index_1 = i32(vertex.atlas_index_1);
    out.atlas_blend = vertex.atlas_blend;

    return out;
}

@fragment
fn fragment(
    in: CustomVertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {

    // If we're in the crossfade section of a visibility range, conditionally
    // discard the fragment according to the visibility pattern.
#ifdef VISIBILITY_RANGE_DITHER
    pbr_functions::visibility_range_dither(in.position, in.visibility_range_dither);
#endif

    // Custom: build vertex data expected by the PBR functions
    var pbr_vertex_data: VertexOutput = pbr_vertex_from_custom_vertex(in);

    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(pbr_vertex_data, is_front);

    // Custom: sample array textures
    var sample_0 = textureSample(custom_array_texture_0, custom_array_texture_sampler_0, in.uv, in.atlas_index_0);
    var sample_1 = textureSample(custom_array_texture_1, custom_array_texture_sampler_1, in.uv, in.atlas_index_1);
    pbr_input.material.base_color = mix(sample_0, sample_1, in.atlas_blend);
#ifdef VERTEX_COLORS
    pbr_input.material.base_color = pbr_input.material.base_color * in.color;
#endif

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    // in forward mode, we calculate the lit color immediately, and then apply some post-lighting effects here.
    // in deferred mode the lit color and these effects will be calculated in the deferred lighting shader
    var out: FragmentOutput;
    if (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u {
        out.color = apply_pbr_lighting(pbr_input);
    } else {
        out.color = pbr_input.material.base_color;
    }

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    return out;
}
