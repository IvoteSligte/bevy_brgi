use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::render_resource::Shader,
};

pub const HANDLE: Handle<Shader> = Handle::weak_from_u128(6244658407449837912);

pub struct CheckIntersectPlugin;

impl Plugin for CheckIntersectPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            HANDLE,
            "../shaders/check_intersect.wgsl",
            Shader::from_wgsl
        );
    }
}
