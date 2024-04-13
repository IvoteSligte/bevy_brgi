#import brgi::utils

@group(0) @binding(0)
var<uniform> param: Params;

// prefix summed offsets for every "pixel" in the orthographic image
// squished from vec2<u32> to u32 so has twice the size of
// count_probes::outerOffsets
@group(0) @binding(1)
var<storage,read> outerOffsets: array<u32>;

@group(0) @binding(2)
var<storage,read> distances: array<f32>;

@group(0) @binding(3)
var<storage,read> transientProbeIndices: array<u32>;

@group(0) @binding(4)
var<storage,write> matchDistance: array<f32>;

@group(0) @binding(5)
var<storage,write> matchIndex: array<i32>;

@compute @workgroup_size(WORKGROUP_LEN)
fn main(@builtin(global_invocation_id) gi: vec3<u32>) {
    let gi: u32 = gi.x;

    if gi >= param.probe_count {
        return;
    }

    let tpi = transientProbeIndices[gi];
    let sig = tpi & 1;
    let dist = distances[pix];
    let otherOuterOffset = outerOffsets[pix][1 - sig];
    // normally might have an out-of-bounds access
    // but it is assumed here that this function is not called for the last element
    // thus making out-of-bounds access impossible
    let otherInnerCount = outerOffsets[pix][1 - sig + 1] - otherOuterOffset;

    var minIndex = -1;
    var minDist = 1e10;

    for (var i: i32 = 0; i < otherInnerCount; i++) {
        let target = otherOuterOffset + i;

        if distances[target] > dist && minDist > distances[target] {
            minIndex = i;
            minDist = distances[target];
        }
    }

    matchDistance[gi] = minDist;
    matchIndex[gi] = minIndex;
}

