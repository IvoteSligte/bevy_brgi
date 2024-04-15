use bevy::prelude::*;
use bevy::render::extract_component::{
    ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
};
use bevy::render::render_graph::{RenderGraphApp, ViewNodeRunner};
use bevy::render::{Render, RenderApp, RenderSet};

use common_cache::{CommonCache, CommonCachePlugin, Params};
use deferred::DeferredPbrLightingPlugin;

pub mod common_cache;
pub mod deferred;
pub mod screen_cache;
pub mod world_cache;

use crate::world_cache::WorldCacheNode;

pub const DEFAULT_PROBE_COUNT: u32 = 32 << 15; // 1 million // must be a multiple of 32 for prefix sum
pub const DEFAULT_IMAGE_LEN: u32 = 128; // resolution
pub const WORKGROUP_SIZE: u32 = 64;
pub const MATERIAL_COUNT: u32 = 256;

pub mod graph {
    use bevy::render::render_graph::{RenderLabel, RenderSubGraph};

    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
    pub struct Brgi;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
    pub enum NodeBrgi {
        World,
        Screen,
    }
}

pub struct BrgiPlugin;

impl Plugin for BrgiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DeferredPbrLightingPlugin,
            CommonCachePlugin,
            // WorldCachePlugin,
            // ScreenCachePlugin,
            ExtractComponentPlugin::<Params>::default(),
            ExtractComponentPlugin::<BrgiCamera>::default(),
            UniformComponentPlugin::<Params>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            error!("Could not fetch render app.");
            return;
        };

        render_app.add_systems(Render, add_render_components); // TODO: look into a better way to
                                                               // do this (and more appropriate schedule)

        render_app.add_render_sub_graph(graph::Brgi);

        render_app.add_render_graph_node::<ViewNodeRunner<WorldCacheNode>>(
            graph::Brgi,
            graph::NodeBrgi::World,
        ); // TODO: screen cache node; connect both nodes
    }
}

#[derive(Component, ExtractComponent, Clone, Default)]
pub struct BrgiCamera;

// FIXME: make brgi components shared between render world and main world

fn add_render_components(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            With<BrgiCamera>,
            Without<CommonCache /* TODO: other caches */>,
        ),
    >,
) {
    for entity in query.iter() {
        let Some(mut entity_commands) = commands.get_entity(entity) else {
            continue;
        };

        entity_commands.insert((
            CommonCache::default(),
            // TODO: other caches
            // TODO: non-default cache values
        ));
    }
}
