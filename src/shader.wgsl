// Vertex shader

struct CameraUniform {
    view_projection_matrix: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, // gl_Position
    @location(0) color: vec3<f32>,
    @location(1) fragment_depth: f32,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.color = model.color;
    let clip_position = camera.view_projection_matrix * vec4<f32>(model.position, 1.0);    
    out.fragment_depth = clip_position.z / clip_position.w;
    out.clip_position = clip_position; 

    return out;
}

// Fragment shader

struct FragmentShaderOutput {
    @location(0) color: vec4<f32>,
    // @builtin(frag_depth) fragDepth: f32,
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentShaderOutput {
    var out: FragmentShaderOutput;

    // out.fragDepth = in.fragment_depth; 
    out.color = vec4<f32>(in.color, 1.0);

    return out;
}

