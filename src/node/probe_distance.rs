use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::render_resource::Shader,
};

pub const HANDLE: Handle<Shader> = Handle::weak_from_u128(13585060688729482803);

pub struct ProbeDistancePlugin;

impl Plugin for ProbeDistancePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            HANDLE,
            "../shaders/probe_distance.wgsl",
            Shader::from_wgsl
        );
    }
}
