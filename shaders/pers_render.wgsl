#import brgi::utils

@binding(0)
var<uniform> param: Params;

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) vertex: Vertex,
}

@vertex
fn vert_main(@location(0) vertex: Probe) -> VertexOutput {
    var out: VertexOutput;
    out.vertex = vertex;
    out.clip_pos = param.pers_world_to_clip * vec4<f32>(vertex.position, 1.0);
}

@fragment
fn frag_main(
    @builtin(position) clip_in: vec4<f32>,
    @builtin(front_facing) front_facing: bool,
    @location(0) vertex: Probe
) -> @location(0) vec4<u32> {
    return packIntersection(vertex.position, vertex.normal_material);
}

