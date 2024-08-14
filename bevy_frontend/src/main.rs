use bevy::prelude::*;
use bevy_frontend::plugins::{
    display::DisplayPlugin,
    // dojo_to_bevy::DojoToBevy,
    // torii::ToriiPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_systems(Startup, default_camera);

    app.add_plugins(DisplayPlugin);

    // app.add_plugins(ToriiPlugin);
    // app.add_plugins(DojoToBevy);

    app.run();
}

fn default_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.3;
    commands.spawn(camera_bundle);
}
