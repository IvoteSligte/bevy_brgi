use bevy::ecs::prelude::*;
use bevy::render::camera::Exposure;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::vec3,
    pbr::{DefaultOpaqueRendererMethod, PbrPlugin},
    prelude::*,
};

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(DefaultOpaqueRendererMethod::deferred())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            brightness: 0.0,
        })
        .add_plugins((
            DefaultPlugins.set(PbrPlugin {
                add_default_deferred_lighting_plugin: false,
                ..default()
            }),
            bevy_flycam::prelude::NoCameraPlayerPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/cornell_box.glb#Scene0"),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 1.0, 4.6).looking_at(vec3(0.0, 1.0, 0.0), Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: std::f32::consts::PI / 6.0,
                near: 0.1,
                far: 1000.0,
                aspect_ratio: 1.0,
            }),
            exposure: Exposure { ev100: 0.0 },
            ..default()
        },
        bevy_flycam::prelude::FlyCam,
    ));
}
