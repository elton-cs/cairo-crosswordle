use bevy::prelude::*;
use starknet::core::utils::parse_cairo_short_string;

use super::torii::BevyEntity;

pub struct DojoToBevy;
impl Plugin for DojoToBevy {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_info_entities);
    }
}

#[derive(Bundle)]
pub struct LetterBundle {
    pub letter: Letter,
    pub position: Position,
    pub letter_status: Status,
}

#[derive(Component)]
pub struct Letter(pub String);

#[derive(Component)]
pub struct Position(pub u8);

#[derive(Component)]
pub enum Status {
    Hidden,
    Solved,
}

fn print_info_entities(mut commands: Commands, query: Query<&BevyEntity>) {
    for dojo_model in query.iter().filter(|e| e.dojo_entity.models.len() > 0) {
        // info!("Building entity {:?}", dojo_model.dojo_entity.models);

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
            .unwrap()
            .to_string();

        info!(letter, position, letter_status_string, letter_status_num);
    }
}

fn build_entities(mut commands: Commands, query: Query<&BevyEntity>) {}
