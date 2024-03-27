#import brgi::utils

@binding(0)
var<uniform> param: Params;

@binding(1)
var<storage,read> probes: array<Probe>;

@binding(2)
var<storage,read_write> counts: array<atomic<u32>>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gi : vec3<u32>) {
    let gi: u32 = gi.x;

    if (gi >= param.probe_count) {
        return;
    }

    let probe: Probe = probes[gi];
    let position: vec3<f32> = probe.position;
    let normal: vec3<f32> = unpack4x8snorm(probe.normal_material).xyz;
    let sig: u32 = u32(dot(normal, param.direction) >= 0.0);
    
    let proj = vec2<u32>((param.world_to_screen * vec4(position)).xy);
    let index = proj.x * param.dimension + proj.y;
    atomicAdd(&counts[index][sig], 1);
}

