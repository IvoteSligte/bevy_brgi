#import brgi::utils

@binding(0)
var<storage,read> inputs: array<u32>;

@binding(1)
var<storage,write> outputs: array<u32>;

var<workgroup> sums: array<u32,WORKGROUP_LEN>;

// sklanksy prefix sum
@compute @workgroup_size(WORKGROUP_LEN)
fn main(@builtin(global_index) gi: vec3<u32>, @builtin(local_index) li: vec3<u32>) {
    let gi: u32 = gi.x;
    let li: u32 = li.x;

    sums[li] = inputs[gi];
    barrier();

    for (var i: u32 = 0; i < 5; i++) {
        // i-th bit is 1
        if bool(extractBits(li, i, 1)) {
            sums[li] += sums[insertBits(li, 0, i, 1) | 1];
        }
        barrier();
    }

    outputs[gi] = sums[li];
}

