#import brgi::utils

@binding(0)
var<uniform> param: Params;

// prefix summed offsets for every "pixel" in the orthographic image
@binding(1)
var<storage,read> outerOffsets: array<vec2<u32>>;

// probe count for every "pixel"
// corresponds to matching_probe::innerOffsets
@binding(2)
var<storage,read> innerCounts: array<vec2<u32>>;

@binding(3)
var<storage,read> distances: array<f32>;

@binding(4)
var<storage,read> matchDistance: array<f32>;

// must be zeroed before this shader is called
@binding(5)
var<storage,write> occlusionBits: array<u32>;

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) vertex: Vertex,
}

@vertex
fn vert_main(@location(0) vertex: Probe) -> VertexOutput {
    var out: VertexOutput;
    out.vertex = vertex;
    out.clip_pos = param.world_to_clip * vec4<f32>(vertex.position, 1.0);
}

@fragment
fn frag_main(
    @builtin(position) clip_in: vec4<f32>,
    @builtin(front_facing) front_facing: bool,
    @location(0) vertex : Probe
) -> @location(0) vec4<u32> {
    clip_in.xyz /= clip_in.w;

    let coords = vec2<i32>(clip_in.xy * f32(param.dimension) - 0.5);
    let dist = clip_in.z;
    let pix = coords.x * param.dimension + coords.y;
    let sig = u32(front_facing);

    var minDist = 1e10;
    var minIndex = -1;

    for (var i = 0; i < innerCounts[pix][1 - sig]; i++) {
        var cmpDist = distances[outerOffsets[pix][1 - sig] + i];
        
        // check if point is on the right side of the intersection based on normal
        // AND that the point is closer than the stored closest point
        if (cmpDist >= dist) == (sig) && bool(u32(cmpDist >= minDist) ^ sig) {
            minDist = cmpDist;
            minIndex = i;
        }
    }

    if minIndex == -1 || matchDistance[outerOffsets[pix][1 - sig] + minIndex] <= dist {
        discard;
    }
    atomicOr(&occlusionBits[minIndex / 32], 1 << minIndex % 32);
    
    return packIntersection(vertex.position, vertex.normal_material);
}

