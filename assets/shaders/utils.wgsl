#define_import_path bevy_brgi::utils

const EPSILON: f32 = 1e-4;
const WORKGROUP_LEN: u32 = 64;

struct Params {
    probe_count: u32, // clamped every frame
    max_probe_count: u32,
}

struct Probe {
    position: vec3<f32>,
    normal: u32,
}

// // representation of intersection, in reality it is a vec4<u32>
// struct Intersection {
//     position: vec3<u32>, // bitcasted vec3<f32>
//     normal_material: u32,
// }

fn packIntersection(position: vec3<f32>, normal_material: u32) -> vec4<u32> {
    return vec4<u32>(bitcast<vec3<u32>>(position), normal_material);
}

fn unpackIntersection(data: vec4<u32>, position: ptr<function,vec3<f32>>, normal_material: ptr<function,u32>) {
    *position = bitcast<vec3<f32>>(data.xyz);
    *normal_material = data.w;
}

fn frag_coord_to_index(coord: vec2<f32>, viewport_width: f32) -> u32 {
    return u32(coord.y * viewport_width + coord.x);
}

