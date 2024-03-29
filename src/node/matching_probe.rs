use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::render_resource::Shader,
};

pub const HANDLE: Handle<Shader> = Handle::weak_from_u128(14658125971807756498);

pub struct MatchingProbePlugin;

impl Plugin for MatchingProbePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            HANDLE,
            "../shaders/matching_probe.wgsl",
            Shader::from_wgsl
        );
    }
}
