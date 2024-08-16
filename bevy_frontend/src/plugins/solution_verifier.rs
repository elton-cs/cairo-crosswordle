use bevy::{input::common_conditions::input_just_pressed, prelude::*, utils::HashMap};

use super::{guess_letter::LetterGuess, manual_bindgen::Letter, mock_torii::spawn_or_update};

pub struct SolutionVerifierPlugin;
impl Plugin for SolutionVerifierPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_solution_verifier);
        app.add_systems(
            Update,
            (set_solution_from_torii, save_completed_guess).after(spawn_or_update),
        );
        app.add_systems(
            PostUpdate,
            verify_solution.run_if(input_just_pressed(KeyCode::Enter)),
        );
    }
}

#[derive(Debug, Resource)]
struct SolutionVerifier {
    solution: HashMap<u8, char>,
    guess: HashMap<u8, char>,
}

fn init_solution_verifier(mut commands: Commands) {
    let solution = HashMap::default();
    let guess = HashMap::default();
    let solution_verifier = SolutionVerifier { solution, guess };
    commands.insert_resource(solution_verifier);
}

fn set_solution_from_torii(
    mut commands: Commands,
    query: Query<&Letter>,
    mut verifier: ResMut<SolutionVerifier>,
) {
    let mut solution: HashMap<u8, char> = HashMap::default();
    let guess: HashMap<u8, char> = HashMap::default();
    for letter in query.iter() {
        solution.insert(letter.position, letter.mock_hash.clone());
    }
    verifier.solution = solution;
}

fn save_completed_guess(
    mut commands: Commands,
    query: Query<&LetterGuess>,
    mut verifier: ResMut<SolutionVerifier>,
) {
    let letter_guess = query.get_single().unwrap();
    let mut guess: HashMap<u8, char> = HashMap::default();
    for (index, letter) in letter_guess.letters.iter().enumerate() {
        guess.insert(index as u8, letter.chars().next().unwrap());
    }
    verifier.guess = guess;
}

fn verify_solution(verifier: Res<SolutionVerifier>) {
    if verifier.guess.len() == 5 {
        if verifier.solution == verifier.guess {
            info!("Solution is correct!");
        } else {
            info!("Solution is incorrect!");
        }
    } else {
        info!("Guess is incomplete!");
    }

    info!("Solution: {:?}", verifier.solution);
    info!("Guess: {:?}", verifier.guess);
}
