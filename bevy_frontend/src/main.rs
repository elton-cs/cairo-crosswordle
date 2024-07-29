use bevy::prelude::*;
use bevy_frontend::plugins::torii::ToriiPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, display_png);
    app.add_plugins(ToriiPlugin);
    app.run();

    // let client = get_torii_client();
    // let position_key = Felt::from_dec_str("1").unwrap();
    // let stream = get_entities_stream(client, position_key);
}

fn display_png(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("keys.png"),
        ..default()
    });
}
