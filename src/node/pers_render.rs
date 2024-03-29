use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::render_resource::Shader,
};

pub const HANDLE: Handle<Shader> = Handle::weak_from_u128(5147757829602892695);

pub struct PersRenderPlugin;

impl Plugin for PersRenderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            HANDLE,
            "../shaders/pers_render.wgsl",
            Shader::from_wgsl
        );
    }
}
