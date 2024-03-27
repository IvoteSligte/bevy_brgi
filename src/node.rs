use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;
use bevy::render::extract_component::ExtractComponent;

pub mod accumulate_probe_color;
pub mod check_intersect;
pub mod count_probes;
pub mod matching_probe;
pub mod pers_render;
pub mod pers_render_probes;
pub mod pers_spawn_intersect_probes;
pub mod prefix_sum_reduce;
pub mod prefix_sum_distribute;
pub mod probe_distance;
pub mod spawn_intersect_probes;

#[derive(Clone, Component, ExtractComponent, ShaderType)]
pub struct Params {
    world_to_screen: Mat4,
    dimension: u32,
    probe_count: u32, // clamped every frame
    max_probe_count: u32,
    direction: Vec3,
    world_to_clip: Mat4,
    screen_to_world: Mat4,
    pers_world_to_clip: Mat4,
    pers_screen_to_world: Mat4,
}

#[derive(Clone, Component, ShaderType)]
pub struct Material {
    dif_rg: u32,
    dif_b_emis_r: u32,
    emis_gb: u32,
}

#[derive(Clone, Component, ShaderType)]
pub struct Probe {
    position: Vec3,
    normal_material: u32,
}

pub mod handle {
    use bevy::prelude::*;

    pub const ACCUMULATE_PROBE_COLOR: Handle<Shader> = Handle::weak_from_u128(1715736618950448815);
    pub const CHECK_INTERSECT: Handle<Shader> = Handle::weak_from_u128(6244658407449837912);
    pub const COUNT_PROBES: Handle<Shader> = Handle::weak_from_u128(6369956254542331318);
    pub const MATCHING_PROBE: Handle<Shader> = Handle::weak_from_u128(14658125971807756498);
    pub const PERS_RENDER: Handle<Shader> = Handle::weak_from_u128(5147757829602892695);
    pub const PERS_RENDER_PROBES: Handle<Shader> = Handle::weak_from_u128(14253784595756508291);
    pub const PERS_SPAWN_INTERSECT_PROBES: Handle<Shader> = Handle::weak_from_u128(14573001387361623342);
    pub const PREFIX_SUM_REDUCE: Handle<Shader> = Handle::weak_from_u128(12144979678946644262);
    pub const PREFIX_SUM_DISTRIBUTE: Handle<Shader> = Handle::weak_from_u128(2377748127963332029);
    pub const PROBE_DISTANCE: Handle<Shader> = Handle::weak_from_u128(13585060688729482803);
    pub const SPAWN_INTERSECT_PROBES: Handle<Shader> = Handle::weak_from_u128(16977374894089826982);
    pub const UTILS: Handle<Shader> = Handle::weak_from_u128(5513461104076943628);
}
