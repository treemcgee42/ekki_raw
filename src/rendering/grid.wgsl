// Vertex shader

struct UnprojectUniform {
    view_projection_matrix: mat4x4<f32>,
    view_projection_matrix_inv: mat4x4<f32>,
    z_near: f32,
    z_far: f32,
};

@group(0) @binding(0)
var<uniform> unproject_uniform: UnprojectUniform;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(1) nearPoint: vec3<f32>,
    @location(2) farPoint: vec3<f32>,
};

fn unproject_point(x :f32, y :f32, z :f32) -> vec3<f32> {
    var unprojectedPoint: vec4<f32> = unproject_uniform.view_projection_matrix_inv * vec4<f32>(x, y, z, 1.0);
    return unprojectedPoint.xyz / unprojectedPoint.w;
}

@vertex
fn vs_main(
    @builtin(vertex_index) vertexIndex : u32
) -> VertexOut {
    var output : VertexOut;
    
    // Grid position
    var gridPlane : array<vec3<f32>,6> = array<vec3<f32>,6>(
        // in xy clipped space
        vec3<f32>(1.0, 1.0, 0.0), vec3<f32>(-1.0, 1.0, 0.0), vec3<f32>(-1.0, -1.0, 0.0),      
        vec3<f32>(-1.0, -1.0, 0.0), vec3<f32>(1.0, -1.0, 0.0), vec3<f32>(1.0, 1.0, 0.0)    
    );
    let p = gridPlane[vertexIndex].xyz;

    // position directly at the clipped coordinates
    output.position = vec4<f32>(p, 1.0);
    // unprojecting on the near plane and far plane
    output.nearPoint = unproject_point(p.x, p.y, 0.0);
    output.farPoint = unproject_point(p.x, p.y, 1.0);

    return output;
}

// S==== Fragment shader {{{1

// `scale` must be integral (TODO: determined the precise restriction, e.g. 0.5 is also 
// valid).
fn grid(fragPos3D: vec3<f32>, scale: f32) -> vec4<f32> {
    // Every fragment (projected onto the XZ-plane) will become a point on the grid.
    let point_on_grid: vec2<f32> = fragPos3D.xz * scale;

    // Affects how thick the lines on the grid are (higher -> thicker).
    let grid_line_scale: f32 = 3.;
    // The derivative on its own specifies the opacity falloff. By modifying it by the
    // grid line scale, we create a affect the thickness / range of the falloff.
    let scaled_derivative: vec2<f32> = grid_line_scale * fwidth(point_on_grid);

    // The reason for the repeated subtraction of 0.5 is to handle negative numbers; 
    // for the purpose of the grid, we don't care about the sign of the points on the
    // grid, e.g. we want to treat -0.8 like we treat 0.2.
    //
    // For a point on a grid line, at least one of the x/z terms is integral, and so the 
    // absolute value term evaluates to 0 and so full opacity is maintained.
    //
    // Dividing by the (scaled) derivative accounts for the scale passed into the function.
    // For a point in the center of a grid square close to the camera, which we would want 
    // to be transparent, the derivative, which would be the "distance" to the point on 
    // the grid the pixel one right one up projects to, would be small, hence this value
    // would be large and would kill the opacity (see below). 
    //
    // For a point close to the grid line and close to the camera, the derivative would be 
    // small again but the absolute value term would also be close to 0, so the result would
    // be a number that is not very large, which would account for the blending we want to 
    // have for anti-aliasing.
    let grid: vec2<f32> = abs(fract(point_on_grid - 0.5) - 0.5) / scaled_derivative;
    let the_line: f32 = min(grid.x, grid.y);
    let minimum_z: f32 = min(scaled_derivative.y, 1.0);
    let minimum_x: f32 = min(scaled_derivative.x, 1.0);
    var color: vec4<f32> = vec4(0.2, 0.2, 0.2, 1.0 - min(the_line, 1.0));

    let axis_scale: f32 = 0.3; // higher -> thicker axis lines
    // z axis
    if (fragPos3D.x > -axis_scale * minimum_x && fragPos3D.x < axis_scale * minimum_x) {
        color.z = 1.0;
    }
    // x axis
    if (fragPos3D.z > -axis_scale * minimum_z && fragPos3D.z < axis_scale * minimum_z) {
        color.x = 1.0;
    }

    return color;
}

struct FragmentShaderOutput {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) fragDepth: f32,
}

@fragment
fn fs_main(in: VertexOut) -> FragmentShaderOutput {
    var output: FragmentShaderOutput;
    
    // Solve for y=0 to determine to point where the fragment projects onto the XZ-plane.
    var t: f32 = -in.nearPoint.y / (in.farPoint.y - in.nearPoint.y);    
    let fragment_on_xz_plane: vec3<f32> = in.nearPoint + t * (in.farPoint - in.nearPoint);
    let fragment_on_xz_plane_clip_space: vec4<f32> = 
        unproject_uniform.view_projection_matrix * vec4<f32>(fragment_on_xz_plane.xyz, 1.0);

    let depth: f32 = fragment_on_xz_plane_clip_space.z / fragment_on_xz_plane_clip_space.w;
    output.fragDepth = depth;
    let transformed_depth: f32 = 2. * depth - 1.; // between -1 and 1
    let linear_depth: f32 = 
        (2.0 * unproject_uniform.z_near * unproject_uniform.z_far) 
        / (
            unproject_uniform.z_far + unproject_uniform.z_near 
            - transformed_depth * (unproject_uniform.z_far - unproject_uniform.z_near)
        );
    let normalized_linear_depth: f32 = linear_depth / unproject_uniform.z_far; 

    let fading_scale: f32 = 5.; // higher -> more fading
    let fading: f32 = max(0., (1. - fading_scale * normalized_linear_depth));
    
    var visibility: f32;
    if (t > 0.0) {
        visibility = 1.0;
    } else {
        visibility = 0.0;
    }
    output.color = grid(fragment_on_xz_plane, 1.) * visibility;
    output.color.w = output.color.w * fading;

    return output;
}

// E==== FRAGMENT SHADER }}}1

