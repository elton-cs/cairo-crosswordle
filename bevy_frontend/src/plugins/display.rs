use bevy::prelude::*;

pub struct DisplayPlugin;
impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, display_png);
    }
}

fn display_png(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("keys.png");

    commands.spawn(SpriteBundle {
        texture: texture_handle,
        ..default()
    });
}
