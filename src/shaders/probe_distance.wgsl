#import brgi::utils

@binding(0)
var<uniform> param: Params;

@binding(1)
var<storage,read> probes: array<vec4<f32>>;

// prefix summed offsets for every "pixel" in the orthographic image
@binding(2)
var<storage,read> outerOffsets: array<vec2<u32>>;

// zeroed offsets for every "pixel" to determine local index
@binding(3)
var<storage,read_write> innerOffsets: array<vec2<u32>>;

@binding(4)
var<storage,write> distances: array<f32>;

@binding(5)
var<storage,write> indices: array<u32>;

@binding(6)
var<storage,write> pixelIndices: array<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gi : vec3<u32>) {
    let gi: u32 = gi.x;
    
    if gi >= param.probe_count {
        return;
    }

    let probe: Probe = probes[gi];
    let position: vec3<f32> = probe.position;
    let normal: vec3<f32> = unpack4x8snorm(probe.normal).xyz;
    let sig: u32 = u32(dot(normal, param.direction) >= 0.0);
    
    let proj = matrix * position;
    let perp = vec2<u32>(proj.xy);
    let dist = proj.z;

    let pix = perp.x * param.dimension + perp.y;
    let inner = atomicAdd(&innerOffsets[pix][sig], 1);
    
    let outer = outerOffsets[pix][sig];
    let total = outer + inner;
    
    distances[total] = dist;
    indices[total] = gi;
    pixelIndices[total] = pix;
}
