#import brgi::utils

@group(0) @binding(0)
var<uniform> param: Params;

@group(0) @binding(1)
var<storage,read> probes: array<Probe>;

@group(0) @binding(2)
var<storage,read> probeColors: array<ProbeColorData>;

@group(0) @binding(3)
var<uniform> intersectTex: texture_storage_2d<vec4<u32>,read>;

@compute @workgroup_size(8,8)
fn main(@builtin(global_invocation_id) gi: vec3<u32>) {
    let gi: vec2<u32> = gi.xy;

    let index = atomicAdd(param.probe_count, 1);

    if index >= param.max_probe_count {
        return;
    }
    let intersect: vec4<u32> = loadTexture(intersectTex, gi);
    let position;
    let normal_material;
    unpackIntersection(intersect, &position, &normal_material);

    let normal = unpack4x8snorm(normal_material).xyz;

    probes[index] = Probe(position + normal * EPSILON, normal_material);
    probeColors[index] = ProbeColorData(vec3<f32>(), normal_material >> 24);
}
