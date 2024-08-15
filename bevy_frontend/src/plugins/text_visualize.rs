use bevy::prelude::*;

use super::manual_bindgen::{Letter, LetterStatus};

pub struct VisualizeTextPlugin;
impl Plugin for VisualizeTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_text_visualizer, update_text_visuals).chain());
    }
}

#[derive(Debug, Component)]
struct TextVisual;

fn add_text_visualizer(
    mut commands: Commands,
    query: Query<(Entity, &Letter, &LetterStatus), Without<TextVisual>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        ..default()
    };
    let text_justification = JustifyText::Center;

    for (entity_id, letter, _letter_status) in query.iter() {
        let visual_text = Text2dBundle {
            text: Text::from_section(letter.mock_hash, text_style.clone())
                .with_justify(text_justification),
            transform: Transform::from_translation(Vec3::new(
                letter.position as f32 * 30.0,
                0.0,
                0.0,
            )),
            ..default()
        };

        commands.entity(entity_id).insert((TextVisual, visual_text));
    }
}

fn update_text_visuals(mut query: Query<(&mut Text, &Letter)>) {
    for (mut text, letter) in query.iter_mut() {
        text.sections[0].value = letter.mock_hash.to_string();
    }
}
