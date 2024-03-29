use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::render_resource::Shader,
};

pub const HANDLE: Handle<Shader> = Handle::weak_from_u128(14573001387361623342);

pub struct PersSpawnIntersectProbesPlugin;

impl Plugin for PersSpawnIntersectProbesPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            HANDLE,
            "../shaders/pers_spawn_intersect_probes.wgsl",
            Shader::from_wgsl
        );
    }
}
