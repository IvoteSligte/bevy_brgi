
@binding(0)
var<storage,read> inputs: array<u32>;

@binding(1)
var<storage,read_write> outputs: array<u32>;

const workgroup_len: u32 = 32;

// sklanksy prefix sum
@compute @workgroup_size(workgroup_len)
fn main(@builtin(global_index) gi: vec3<u32>, @builtin(workgroup_id) wi : vec3<u32>) {
    let gi: u32 = gi.x;
    let wi: u32 = wi.x;

    outputs[gi] += inputs[wi];
}

