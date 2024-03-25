#import brgi::utils

@binding(2)
var<storage,read> probeColors: array<ProbeColorData>;

@binding(3)
var<uniform> intersectTex: texture_storage_2d<vec4<u32>,read>;

// // output of check_intersect.wgsl fragment
// struct FragmentOutput {
//     position: vec3<u32>, // bitcasted vec3<f32>
//     normal_material: u32,
// }

@compute @workgroup_size(8,8)
fn main(@builtin(global_invocation_id) gi : vec3<u32>) {
    let gi: vec2<u32> = gi.xy;

    
    if index >= param.max_probe_count {
        return;
    }

    // this is temporary, in the future there should be a check
    // for whether or not a probe should be spawned due to surrounding density
    let intersect: vec4<u32> = loadTexture(intersectTex, gi);
    let position;
    let normal_material;
    unpackIntersection(intersect, &position, &normal_material);

    let normal = unpack4x8snorm(normal_material).xyz;

    probes[index] = Probe(position + normal * EPSILON, normal_material);
    probeColors[index] = ProbeColorData(vec3<f32>(), normal_material >> 24);
}
