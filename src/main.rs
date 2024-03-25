use bevy::{asset::load_internal_asset, prelude::*};

pub mod node;

pub mod graph {
    pub const NAME: &str = "brgi";
    
    pub mod node {
        pub const ACCUMULATE_PROBE_COLOR: &str = "accumulate_probe_color";
        pub const CHECK_INTERSECT: &str = "check_intersect";
        pub const COUNT_PROBES: &str = "count_probes";
        pub const INTERSECTION_TO_COLOR: &str = "intersection_to_color";
        pub const MATCHING_PROBE: &str = "matching_probe";
        pub const PERS_RENDER: &str = "pers_render";
        pub const PERS_RENDER_PROBES: &str = "pers_render_probes";
        pub const PERS_SPAWN_INTERSECT_PROBES: &str = "pers_spawn_intersect_probes";
        pub const PREFIX_SUM: &str = "prefix_sum";
        pub const PROBE_DISTANCE: &str = "probe_distance";
        pub const SPAWN_INTERSECT_PROBES: &str = "spawn_intersect_probes";
    }
}

pub mod shader {
    use bevy::prelude::*;
    use bevy::render::render_resource::ShaderType;

    #[derive(Clone, Component, ShaderType)]
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
        pub const INTERSECTION_TO_COLOR: Handle<Shader> = Handle::weak_from_u128(16550955340454056218);
        pub const MATCHING_PROBE: Handle<Shader> = Handle::weak_from_u128(14658125971807756498);
        pub const PERS_RENDER: Handle<Shader> = Handle::weak_from_u128(5147757829602892695);
        pub const PERS_RENDER_PROBES: Handle<Shader> = Handle::weak_from_u128(14253784595756508291);
        pub const PERS_SPAWN_INTERSECT_PROBES: Handle<Shader> = Handle::weak_from_u128(14573001387361623342);
        pub const PREFIX_SUM: Handle<Shader> = Handle::weak_from_u128(12144979678946644262);
        pub const PROBE_DISTANCE: Handle<Shader> = Handle::weak_from_u128(13585060688729482803);
        pub const SPAWN_INTERSECT_PROBES: Handle<Shader> = Handle::weak_from_u128(16977374894089826982);
        pub const UTILS: Handle<Shader> = Handle::weak_from_u128(5513461104076943628);
    }
}

struct BrgiPlugin;

impl Plugin for BrgiPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            shader::handle::ACCUMULATE_PROBE_COLOR,
            "../shaders/accumulate_probe_color.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            shader::handle::CHECK_INTERSECT,
            "../shaders/check_intersect.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            shader::handle::COUNT_PROBES,
            "../shaders/count_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            shader::handle::INTERSECTION_TO_COLOR,
            "../shaders/intersection_to_color.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            shader::handle::MATCHING_PROBE,
            "../shaders/matching_probe.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            shader::handle::PERS_RENDER,
            "../shaders/pers_render.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            shader::handle::PERS_RENDER_PROBES,
            "../shaders/pers_render_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            shader::handle::PERS_SPAWN_INTERSECT_PROBES,
            "../shaders/pers_spawn_intersect_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            shader::handle::PREFIX_SUM,
            "../shaders/prefix_sum.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            shader::handle::PROBE_DISTANCE,
            "../shaders/probe_distance.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            shader::handle::SPAWN_INTERSECT_PROBES,
            "../shaders/spawn_intersect_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            shader::handle::UTILS,
            "../shaders/utils.wgsl",
            Shader::from_wgsl
        );
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, BrgiPlugin)).run();
}
