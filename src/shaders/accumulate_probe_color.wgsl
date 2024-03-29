#import brgi::utils

@binding(0)
var<uniform> param: Params;

@binding(1)
var<storage,read> probes: array<Probe>;

@binding(2)
var<storage,read> probeColors: array<ProbeColorData>;

@binding(3)
var<uniform> materials: array<Material>;

@binding(2)
var<storage,read> matchIndices: array<i32>;

@binding(3)
var<storage,read> occlusionBits: array<u32>;

// true indices, maps the indices in the transient space to
// the actual probe position/normal/color/material indices
@binding(4)
var<storage,write> indices: array<u32>;

fn unpackColorData(data: ProbeColorData, color: ptr<function,vec3<f32>>, matIndex: ptr<function,u32>) {
    *color.rg = unpack2x16float(data.colorRG);
    *color.b = unpack2x16float(data.colorB_material).x;
    *matIndex = data.colorB_material >> 16;
}

fn unpackMaterial(mat: Material, diffuse: ptr<function,vec3<f32>>, emission: ptr<function,vec3<f32>>) {
    *diffuse.rg = unpack2x16float(mat.difRG);
    *diffuse.b = unpack2x16float(mat.difB_emisR).x;
    *emission.r = unpack2x16float(mat.difB_emisR).y;
    *emission.gb = unpack2x16float(mat.emisGB);
}

@compute @workgroup_size(WORKGROUP_LEN)
fn main(@builtin(global_invocation_id) gi: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    let gi: u32 = gi.x;

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
    let tmIndex = indices[mIndex]; // true match index
    let mColor;
    let mMatIndex;
    unpackColorData(probeColors[tmIndex], &mColor, &mMaterial);

    let mMat = materials[mMatIndex];
    let diffuse: vec3<f32>;
    let emission: vec3<f32>;
    unpackMaterial(matchMat, &diffuse, &emission);

    let ti = indices[gi]; // true index
    let normal: vec3<f32> = unpack4x8snorm(probes[ti].normal).xyz;
    let coef = abs(dot(normal, param.direction));

    var color;
    let material;
    unpackColorData(probeColors[ti], &color, &material);

    color += emission * coef;
    color += matchColorData.color * diffuse * coef;

    probeColors[ti].color = color;
}
