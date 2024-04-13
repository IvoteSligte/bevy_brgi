use bevy::prelude::*;
use bevy_brgi::BrgiPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, BrgiPlugin)).run();
}
