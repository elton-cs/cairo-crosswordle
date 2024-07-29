#[cfg(test)]
mod tests {
    use dojo::world::{IWorldDispatcher, IWorldDispatcherTrait};
    use dojo::utils::test::{spawn_test_world, deploy_contract};
    use dojo_starter::{
        systems::create::{create_actions, ICreateDispatcher, ICreateDispatcherTrait},
        models::board::{Letter, letter, LetterStatus, letter_status, Status},
    };

    fn setup() -> (IWorldDispatcher, ICreateDispatcher) {
        // world setup
        let mut models = array![letter::TEST_CLASS_HASH, letter_status::TEST_CLASS_HASH];
        let world = spawn_test_world("dojo_starter", models);
        let contract_address = world
            .deploy_contract(
                'salt', create_actions::TEST_CLASS_HASH.try_into().unwrap(), array![].span()
            );
        let systems = ICreateDispatcher { contract_address };

        (world, systems)
    }

    // #[test]
    // fn test_create_single_letter() {
    //     let (world, systems) = setup();
    //     let caller = starknet::contract_address_const::<0x0>();

    //     // create single letter
    //     let character = 'N';
    //     let position = 1;
    //     systems.create_letter(character, position);

    //     // test
    //     let single_letter = get!(world, 1, (Letter));
    //     assert!(single_letter.position == 1, "Letter is placed in wrong position");
    //     assert!(single_letter.hash == 'N'.into(), "Letter is incorrect");
    //     assert!(single_letter.placed_by == caller, "Wrong player");
    // }

    // #[test]
    // fn test_create_multiple_letters() {
    //     let (world, systems) = setup();
    //     let caller = starknet::contract_address_const::<0x0>();

    //     // create multiple letters
    //     let characters = ['N', 'I', 'N', 'J', 'A'].span();
    //     let mut index = 0;
    //     while (index < 5) {
    //         systems.create_letter(characters[index].clone(), index.try_into().unwrap());
    //         index += 1;
    //     };

    //     // test
    //     index = 0;
    //     while (index < 5) {
    //         let position: u8 = index.try_into().unwrap();
    //         let single_letter = get!(world, position, (Letter));
    //         assert!(single_letter.hash == characters[index].clone(), "Letter is incorrect");
    //         assert!(single_letter.placed_by == caller, "Wrong player");
    //         index += 1;
    //     };
    // }

    // #[test]
    // fn test_word() {
    //     let (world, systems) = setup();

    //     let single_word = ['N', 'I', 'N', 'J', 'A'];
    //     systems.create_word(single_word);

    //     // test
    //     let mut index = 0;
    //     let word_span = single_word.span();
    //     while index < word_span
    //         .len() {
    //             let position: u8 = index.try_into().unwrap();
    //             let letter = get!(world, position, (Letter));
    //             assert_eq!(letter.hash, word_span[index].clone());
    //             index += 1;
    //         }
    // }

    // #[test]
    // fn test_create_hidden_word() {
    //     let (world, systems) = setup();

    //     let single_word = ['N', 'I', 'N', 'J', 'A'];
    //     systems.create_word(single_word);

    //     // test
    //     let mut index = 0;
    //     let expected_status: felt252 = Status::Hidden.into();
    //     let word_span = single_word.span();
    //     while index < word_span
    //         .len() {
    //             let position: u8 = index.try_into().unwrap();
    //             let (letter, letter_status) = get!(world, position, (Letter, LetterStatus));
    //             // assert the letter retrieved from the world matches
    //             assert_eq!(letter.hash, word_span[index].clone());
    //             // assert that the status of each letter is Hidden
    //             let curr_status: felt252 = letter_status.status.into();
    //             assert_eq!(curr_status, expected_status);
    //             index += 1;
    //         }
    // }

    #[test]
    fn test_solve_letter_with_right_guess() {
        let (world, systems) = setup();

        let single_word = ['N', 'I', 'N', 'J', 'A'];
        let word_span = single_word.span();
        let hidden_status: felt252 = Status::Hidden.into();
        let solved_status: felt252 = Status::Solved.into();

        // create a hidden word
        systems.create_hidden_word(single_word);
        let pos: u32 = 0;

        // verify letter and its hidden status
        let (letter, status) = get!(world, pos, (Letter, LetterStatus));
        let status_in_felt: felt252 = status.status.into();
        assert_eq!(letter.hash, word_span[pos].clone());
        assert_eq!(status_in_felt, hidden_status);

        // solve letter at position 0 aka 'N'
        systems.solve_letter('N', pos.try_into().unwrap());

        // verify letter and its solved status
        let (letter, status) = get!(world, pos, (Letter, LetterStatus));
        let status_in_felt: felt252 = status.status.into();
        assert_eq!(letter.hash, word_span[pos].clone());
        assert_eq!(status_in_felt, solved_status);
    }

    #[test]
    fn test_solve_letter_with_wrong_guess() {
        let (world, systems) = setup();

        let single_word = ['N', 'I', 'N', 'J', 'A'];
        let word_span = single_word.span();
        let hidden_status: felt252 = Status::Hidden.into();

        // create a hidden word
        systems.create_hidden_word(single_word);
        let pos: u32 = 0;

        // verify letter and its hidden status
        let (letter, status) = get!(world, pos, (Letter, LetterStatus));
        let status_in_felt: felt252 = status.status.into();
        assert_eq!(letter.hash, word_span[pos].clone());
        assert_eq!(status_in_felt, hidden_status);

        // attempt to solve letter with incorrect guess: 'I'
        // THIS WILL PANIC AND THE TEST IS EXPECTED TO FAIL
        systems.solve_letter('I', pos.try_into().unwrap());
    }
}
