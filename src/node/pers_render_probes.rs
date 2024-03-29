use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::render_resource::Shader,
};

pub const HANDLE: Handle<Shader> = Handle::weak_from_u128(14253784595756508291);

pub struct PersRenderProbesPlugin;

impl Plugin for PersRenderProbesPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            HANDLE,
            "../shaders/pers_render_probes.wgsl",
            Shader::from_wgsl
        );
    }
}
