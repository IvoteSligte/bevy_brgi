use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::render_resource::Shader,
};

pub const HANDLE_REDUCE: Handle<Shader> = Handle::weak_from_u128(12144979678946644262);
pub const HANDLE_DISTRIBUTE: Handle<Shader> = Handle::weak_from_u128(2377748127963332029);

pub struct PrefixSumPlugin;

impl Plugin for PrefixSumPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            HANDLE_REDUCE,
            "../shaders/prefix_sum_reduce.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            HANDLE_DISTRIBUTE,
            "../shaders/prefix_sum_distribute.wgsl",
            Shader::from_wgsl
        );
    }
}
