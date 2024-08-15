use bevy::prelude::*;
use rand::Rng;

use super::manual_bindgen::{Letter, LetterStatus, Status};

pub struct ToriiPlugin;
impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, mock_word_entities);
        app.add_systems(Update, spawn_or_update);
    }
}

#[derive(Debug, Component)]
struct TempDojoEntity {
    letter: Letter,
    letter_status: LetterStatus,
}

fn mock_word_entities(mut commands: Commands) {
    let random_5_letter_words = [
        "apple", "beach", "candy", "dough", "eagle", "fairy", "giant", "happy", "igloo", "jelly",
        "kitty", "lucky", "mango", "noble", "olive", "panda", "queen", "robin", "sunny", "tiger",
        "umbra", "vivid", "wacky", "xenon", "yacht", "zebra",
    ];

    // random number between 0 and 25:
    let random_index = rand::thread_rng().gen_range(0..26);

    let word = random_5_letter_words[random_index];

    for (index, letter) in word.chars().enumerate() {
        let letter = Letter {
            position: index as u8,
            mock_hash: letter.clone(),
        };

        let letter_status = LetterStatus {
            position: index as u8,
            status: Status::Hidden,
        };

        commands.spawn(TempDojoEntity {
            letter,
            letter_status,
        });
    }
}

fn spawn_or_update(
    mut commands: Commands,
    mut query_dojo_entity: Query<(Entity, &mut TempDojoEntity)>,
    mut query_bevy_entity: Query<(&mut Letter, &mut LetterStatus)>,
) {
    for (id, dojo_entity) in query_dojo_entity.iter_mut() {
        // values for the Letter component
        let position = dojo_entity.letter.position;
        let mock_hash = dojo_entity.letter.mock_hash;

        // values for the LetterStatus component
        let status = dojo_entity.letter_status.status.clone();

        let mut is_new = true;

        for (mut existing_letter, mut existing_letter_status) in query_bevy_entity.iter_mut() {
            if existing_letter.position == position {
                existing_letter.mock_hash = mock_hash;
                existing_letter_status.status = status.clone();
                is_new = false;
            }
        }
        if is_new {
            let letter = dojo_entity.letter.clone();
            let letter_status = dojo_entity.letter_status.clone();

            commands.spawn((letter, letter_status));
        }

        commands.entity(id).despawn();
    }
}
