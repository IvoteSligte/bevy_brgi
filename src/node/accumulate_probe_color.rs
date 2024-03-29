use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::render_resource::Shader,
};

pub const HANDLE: Handle<Shader> = Handle::weak_from_u128(1715736618950448815);

pub struct AccumulateProbeColorPlugin;

impl Plugin for AccumulateProbeColorPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            HANDLE,
            "../shaders/accumulate_probe_color.wgsl",
            Shader::from_wgsl
        );
    }
}
