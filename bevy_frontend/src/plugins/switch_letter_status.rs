use super::manual_bindgen::{LetterStatus, Status};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub struct FlipLetterStatusPlugin;
impl Plugin for FlipLetterStatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            flip_letter_status.run_if(input_just_pressed(KeyCode::Space)),
        );
    }
}

fn flip_letter_status(mut query: Query<&mut LetterStatus>) {
    for mut letter_status in query.iter_mut() {
        letter_status.status = match letter_status.status {
            Status::Hidden => Status::Solved,
            Status::Solved => Status::Hidden,
        };
    }
}
