use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::render_resource::Shader,
};

pub const HANDLE: Handle<Shader> = Handle::weak_from_u128(16977374894089826982);

pub struct SpawnIntersectProbesPlugin;

impl Plugin for SpawnIntersectProbesPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            HANDLE,
            "../shaders/spawn_intersect_probes.wgsl",
            Shader::from_wgsl
        );
    }
}
