use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::render_resource::Shader,
};

pub const HANDLE: Handle<Shader> = Handle::weak_from_u128(5513461104076943628);

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, HANDLE, "../shaders/utils.wgsl", Shader::from_wgsl);
    }
}
