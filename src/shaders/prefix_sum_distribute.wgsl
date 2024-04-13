#import brgi::utils

@group(0) @binding(0)
var<storage,read> inputs: array<u32>;

@group(0) @binding(1)
var<storage,read_write> outputs: array<u32>;

// sklanksy prefix sum
@compute @workgroup_size(WORKGROUP_LEN)
fn main(@builtin(global_index) gi: vec3<u32>, @builtin(workgroup_id) wi: vec3<u32>) {
    let gi: u32 = gi.x;
    let wi: u32 = wi.x;

    outputs[gi] += inputs[wi];
}

