use bevy::prelude::*;
use starknet::core::utils::parse_cairo_short_string;

use super::torii::BevyEntity;

pub struct DojoToBevy;
impl Plugin for DojoToBevy {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, build_bevy_entities);
    }
}

#[derive(Component, Debug)]
pub struct LetterPiece {
    pub letter: String,
    pub position: u8,
    pub letter_status: Status,
}

#[derive(Debug)]
pub enum Status {
    Hidden,
    Solved,
}

fn build_bevy_entities(
    mut commands: Commands,
    torii_query: Query<&BevyEntity>,
    mut letter_query: Query<&mut LetterPiece>,
    asset_server: Res<AssetServer>,
) {
    for dojo_model in torii_query
        .iter()
        .filter(|e| e.dojo_entity.models.len() > 0)
    {
        // info!("Building entity {:?}", dojo_model.dojo_entity.models);

        let (torii_letter, torii_position, _, torii_letter_status_num) =
            extract_dojo_model_data(dojo_model);

        let letter_status = match torii_letter_status_num {
            0 => Status::Hidden,
            1 => Status::Solved,
            _ => panic!("Invalid letter status"),
        };

        if let Some(mut bevy_letter_piece) = letter_query
            .iter_mut()
            .find(|e| e.position == torii_position)
        {
            bevy_letter_piece.letter = torii_letter;
            bevy_letter_piece.letter_status = letter_status;
        } else {
            info!("New Dojo to Bevy Conversion Entity:");
            let letter_status_string = match letter_status {
                Status::Hidden => "hidden",
                Status::Solved => "solved",
            };
            info!(torii_letter, torii_position, letter_status_string);

            let new_letter_piece = LetterPiece {
                letter: torii_letter.clone(),
                position: torii_position.clone(),
                letter_status,
            };

            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            let text_style = TextStyle {
                font: font.clone(),
                font_size: 60.0,
                ..default()
            };
            let text_justification = JustifyText::Center;

            let visual_text = Text2dBundle {
                text: Text::from_section(torii_letter, text_style.clone())
                    .with_justify(text_justification),
                transform: Transform::from_translation(Vec3::new(
                    torii_position as f32 * 30.0,
                    0.0,
                    0.0,
                )),
                ..default()
            };

            commands.spawn((new_letter_piece, visual_text));
        }
    }
}

fn extract_dojo_model_data(dojo_model: &BevyEntity) -> (String, u8, String, u8) {
    let letter = dojo_model.dojo_entity.models[1].children[1]
        .ty
        .as_primitive()
        .unwrap()
        .as_felt252()
        .unwrap();
    let letter = parse_cairo_short_string(&letter).unwrap();

    let pre_letter_status = dojo_model.dojo_entity.models[0].children[1]
        .ty
        .as_enum()
        .unwrap();
    let letter_status_string = pre_letter_status.options[0].name.to_string();
    let letter_status_num = pre_letter_status.option.unwrap();

    let position = dojo_model.dojo_entity.models[1].children[0]
        .ty
        .as_primitive()
        .unwrap()
        .as_u8()
        .unwrap();

    // info!(letter, position, letter_status_string, letter_status_num);

    (letter, position, letter_status_string, letter_status_num)
}
