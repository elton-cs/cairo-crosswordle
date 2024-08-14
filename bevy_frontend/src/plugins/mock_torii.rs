use bevy::prelude::*;

use super::manual_bindgen::{Letter, LetterStatus, Status};

pub struct ToriiPlugin;
impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, mock_word_entities);
    }
}

#[derive(Debug, Component)]
struct TempDojoEntity {
    letter: Letter,
    letter_status: LetterStatus,
}

fn mock_word_entities(mut commands: Commands) {
    let word = ['N', 'I', 'N', 'J', 'A'];

    for (index, letter) in word.iter().enumerate() {
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
