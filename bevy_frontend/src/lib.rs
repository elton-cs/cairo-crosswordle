use bevy::prelude::*;
use plugins::{
    cursor::CursorPlugin, guess_letter::GuessLetterPlugin, image_visualize::VisualizeImagePlugin,
    mock_torii::ToriiPlugin, solution_verifier::SolutionVerifierPlugin,
    switch_letter_status::FlipLetterStatusPlugin,
};

pub mod plugins;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
        app.add_systems(Startup, default_camera);
        // app.add_systems(Update, display_entity_count);

        app.add_plugins(ToriiPlugin);
        app.add_plugins(VisualizeImagePlugin);
        app.add_plugins(FlipLetterStatusPlugin);
        app.add_plugins(CursorPlugin);
        app.add_plugins(GuessLetterPlugin);
        app.add_plugins(SolutionVerifierPlugin);
    }
}

fn default_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.3;
    commands.spawn(camera_bundle);
}
