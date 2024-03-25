#import brgi::utils

@binding(0)
var<uniform> param: Params;

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) vertex: Vertex,
    @location(4) probeIndex: u32,
}

@vertex
fn vert_main(@builtin(vertex_index) vertex_index: u32, @location(0) vertex: Probe) -> VertexOutput {
    var out: VertexOutput;
    out.vertex = vertex;
    out.probeIndex = vertex_index;
    out.clip_pos = param.pers_world_to_clip * vec4<f32>(vertex.position, 1.0);
}

struct FragmentOutput {
    @location(0) probe: vec4<u32>,
    @location(4) probeIndex: u32,
}

@fragment
fn frag_main(
    @builtin(position) clip_in: vec4<f32>,
    @builtin(front_facing) front_facing: bool,
    @location(0) vertex: Probe
    @location(4) probeIndex: u32,
) -> FragmentOutput {
    return packIntersection(vertex.position, vertex.normal_material);
}

