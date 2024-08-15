use bevy::prelude::*;
use bevy_frontend::plugins::{
    // display::DisplayPlugin,
    // dojo_to_bevy::DojoToBevy,
    // torii::ToriiPlugin,
    mock_torii::ToriiPlugin,
    visualize::VisualizePlugin,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_systems(Startup, default_camera);
    app.add_systems(Update, display_entity_count);
    app.add_plugins(ToriiPlugin);
    app.add_plugins(VisualizePlugin);
    // app.add_plugins(DisplayPlugin);
    // app.add_plugins(ToriiPlugin);
    // app.add_plugins(DojoToBevy);

    app.run();
}

fn default_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.3;
    commands.spawn(camera_bundle);
}

fn display_entity_count(query: Query<Entity>) {
    let mut total = 0;
    for _entity in query.iter() {
        total += 1;
    }
    info!("Total entities: {}", total);
}
