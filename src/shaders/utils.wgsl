
const EPSILON: f32 = 1e-4;

struct Params {
    world_to_screen: mat3x3<f32>,
    dimension: u32,
    probe_count: u32, // clamped every frame
    max_probe_count: u32,
    direction: vec3<f32>,
    world_to_clip: mat4x4<f32>,
    screen_to_world: mat4x4<f32>,
    pers_world_to_clip: mat4x4<f32>,
    pers_screen_to_world: mat4x4<f32>,
}

struct Probe {
    position: vec3<f32>,
    normal_material: u32,
}

struct ProbeColorData {
    colorRG: u32,
    colorB_material: u32,
}

struct Material {
    difRG: u32,
    difB_emisR: u32,
    emisGB: u32,
}

// // representation of intersection, in reality it is a vec4<u32>
// struct Intersection {
//     position: vec3<u32>, // bitcasted vec3<f32>
//     normal_material: u32,
// }

fn extractProbeColorDataColor(data: ProbeColorData) -> vec3<f32> {
    var color: vec3<f32>;

    color.rg = unpack2x16float(data.colorRG);
    color.b = unpack2x16float(data.colorB_material).x;

    return color;
}

fn packIntersection(position: vec3<f32>, normal_material: u32) -> vec4<u32> {
    return vec4<u32>(bitcast<vec3<u32>>(position), normal_material);
}

fn unpackIntersection(data: vec4<u32>, position: ptr<function,vec3<f32>>, normal_material: ptr<function,u32>) {
    *position = bitcast<vec3<f32>>(data.xyz);
    *normal_material = data.w;
}

fn unpackNormalMaterial(data: u32, normal: ptr<function,vec3<f32>>, material: ptr<function,u32>) {
    *normal = unpack4x8snorm(data).xyz;
    *material = data >> 24;
}

