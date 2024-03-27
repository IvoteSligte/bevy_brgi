use bevy::{asset::load_internal_asset, prelude::*};
use bevy::render::extract_component::{ExtractComponentPlugin, UniformComponentPlugin};
use bevy::render::render_graph::ViewNodeRunner;
use bevy::render::RenderApp;
use bevy::render::render_graph::RenderGraphApp;

pub mod node;

use node::count_probes::*;

pub mod graph {
    use bevy::render::render_graph::{RenderLabel, RenderSubGraph};

    pub const NAME: &str = "brgi";

    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
    pub struct Brgi;

    pub mod node {
        pub const ACCUMULATE_PROBE_COLOR: &str = "accumulate_probe_color";
        pub const CHECK_INTERSECT: &str = "check_intersect";
        pub const COUNT_PROBES: &str = "count_probes";
        pub const MATCHING_PROBE: &str = "matching_probe";
        pub const PERS_RENDER: &str = "pers_render";
        pub const PERS_RENDER_PROBES: &str = "pers_render_probes";
        pub const PERS_SPAWN_INTERSECT_PROBES: &str = "pers_spawn_intersect_probes";
        pub const PREFIX_SUM: &str = "prefix_sum";
        pub const PROBE_DISTANCE: &str = "probe_distance";
        pub const SPAWN_INTERSECT_PROBES: &str = "spawn_intersect_probes";
    }

    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
    pub enum NodeBrgi {
        CountProbes,
        PrefixSum,
        ProbeDistance,
        CheckIntersect,
        SpawnIntersectProbes,
        PersRender,
        PersRenderProbes,
        PersSpawnIntersectProbes,
        AccumulateProbeColor,
    }
}

struct BrgiPlugin;

impl Plugin for BrgiPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            node::handle::ACCUMULATE_PROBE_COLOR,
            "shaders/accumulate_probe_color.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            node::handle::CHECK_INTERSECT,
            "shaders/check_intersect.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            node::handle::COUNT_PROBES,
            "shaders/count_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            node::handle::MATCHING_PROBE,
            "shaders/matching_probe.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            node::handle::PERS_RENDER,
            "shaders/pers_render.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            node::handle::PERS_RENDER_PROBES,
            "shaders/pers_render_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            node::handle::PERS_SPAWN_INTERSECT_PROBES,
            "shaders/pers_spawn_intersect_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            node::handle::PREFIX_SUM_REDUCE,
            "shaders/prefix_sum_reduce.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            node::handle::PREFIX_SUM_DISTRIBUTE,
            "shaders/prefix_sum_distribute.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            node::handle::PROBE_DISTANCE,
            "shaders/probe_distance.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            node::handle::SPAWN_INTERSECT_PROBES,
            "shaders/spawn_intersect_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            node::handle::UTILS,
            "shaders/utils.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins((ExtractComponentPlugin::<node::Params>::default(), UniformComponentPlugin::<node::Params>::default()));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            error!("Could not fetch render app.");
            return;
        };

        render_app.add_render_graph_node::<ViewNodeRunner<CountProbesNode>>(graph::Brgi, graph::NodeBrgi::CountProbes);

        todo!()
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            error!("Could not fetch render app.");
            return;
        };

        render_app.init_resource::<CountProbesPipeline>();    
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, BrgiPlugin)).run();
}
