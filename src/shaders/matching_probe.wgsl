#import brgi::utils

@binding(0)
var<uniform> param: Params;

// prefix summed offsets for every "pixel" in the orthographic image
// squished from vec2<u32> to u32 so has twice the size of
// count_probes::outerOffsets
@binding(1)
var<storage,read> outerOffsets: array<u32>;

// probe count for every "pixel"
// squished from vec2<u32> to u32 so has twice the size of
// count_probes::innerOffsets
@binding(2)
var<storage,read> innerCounts: array<u32>;

@binding(3)
var<storage,read> distances: array<f32>;

@binding(4)
var<storage,read> transientProbeIndices: array<u32>;

@binding(5)
var<storage,write> matchDistance: array<f32>;

@binding(6)
var<storage,write> matchIndex: array<i32>;

@compute @workgroup_size(WORKGROUP_LEN)
fn main(@builtin(global_invocation_id) gi: vec3<u32>) {
    let gi: u32 = gi.x;

    if gi >= param.probe_count {
        return;
    }

    let tpi = transientProbeIndices[gi];
    let otherTpi = tpi ^ 1;
    let dist = distances[pix];
    var minIndex = -1;
    var minDist = 1e10;

    for (var i: i32 = 0; i < innerCounts[otherTpi]; i++) {
        let target = outerOffsets[otherTpi] + i;

        if distances[target] > dist && minDist > distances[target] {
            minIndex = i;
            minDist = distances[target];
        }
    }

    matchDistance[tpi] = minDist;
    matchIndex[tpi] = minIndex;
}

