#import brgi::utils

@group(0) @binding(0)
var<uniform> param: utils::Params;

@group(0) @binding(1)
var<storage,read> probes: array<utils::Probe>;

@group(0) @binding(2)
var<storage,read_write> probeColors: array<utils::ProbeColorData>;

@group(0) @binding(3)
var<uniform> materials: array<utils::Material, MATERIAL_COUNT>;

@group(0) @binding(2)
var<storage,read> matchIndices: array<i32>;

@group(0) @binding(3)
var<storage,read> occlusionBits: array<u32>;

// true indices, maps the indices in the transient space to
// the actual probe position/normal/color/material indices
@group(0) @binding(4)
var<storage,read> indices: array<u32>;

@compute @workgroup_size(utils::WORKGROUP_LEN)
fn main(@builtin(global_invocation_id) giv: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    let gi: u32 = giv.x;

    if gi >= param.probe_count {
        return;
    }
    if bool(occlusionBits[gi / 32] & (1 << li)) {
        return;
    }
    let mIndex = matchIndices[gi];

    if mIndex == -1 {
        return;
    }
    let tmi = indices[mIndex]; // true match index
    var mColor: vec3<f32>;
    var mMatIndex: u32;
    utils::unpackColorData(probeColors[tmi], &mColor, &mMatIndex);

    let mMat = materials[mMatIndex];
    var diffuse: vec3<f32>;
    var emission: vec3<f32>;
    utils::unpackMaterial(mMat, &diffuse, &emission);

    let ti = indices[gi]; // true index
    let normal: vec3<f32> = unpack4x8snorm(probes[ti].normal_material).xyz;
    let coef = abs(dot(normal, param.direction));

    var color: vec3<f32>;
    var material: u32;
    utils::unpackColorData(probeColors[ti], &color, &material);

    color += emission * coef;
    color += mColor * diffuse * coef;

    probeColors[ti] = utils::packColorData(color, material);
}
