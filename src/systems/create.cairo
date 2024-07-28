use dojo_starter::models::board::Letter;
const NUM_LETTERS: u32 = 5;

#[dojo::interface]
trait ICreate {
    fn create_letter(ref world: IWorldDispatcher, character: felt252, position: u8);
    fn create_word(ref world: IWorldDispatcher, word: [felt252; NUM_LETTERS]);
    fn create_hidden_word(ref world: IWorldDispatcher, word: [felt252; NUM_LETTERS]);
}

#[dojo::contract]
mod create_actions {
    use super::ICreate;
    use super::NUM_LETTERS;
    use starknet::{ContractAddress, get_caller_address};
    use dojo_starter::models::board::{Letter, Status, Letter_Status};


    #[abi(embed_v0)]
    impl CreateImpl of ICreate<ContractState> {
        fn create_letter(ref world: IWorldDispatcher, character: felt252, position: u8) {
            let placed_by = get_caller_address();
            let single_letter = Letter { position, hash: character, placed_by, };

            set!(world, (single_letter));
        }

        fn create_word(ref world: IWorldDispatcher, word: [felt252; NUM_LETTERS]) {
            let placed_by = get_caller_address();
            let word_span = word.span();

            let mut index = 0;
            while index < NUM_LETTERS {
                let position = index.try_into().unwrap();
                let single_letter = Letter { position, hash: word_span[index].clone(), placed_by };
                set!(world, (single_letter));

                index += 1;
            }
        }

        fn create_hidden_word(ref world: IWorldDispatcher, word: [felt252; NUM_LETTERS]) {
            let placed_by = get_caller_address();
            let word_span = word.span();

            let mut index = 0;
            while index < NUM_LETTERS {
                let position = index.try_into().unwrap();
                let single_letter = Letter { position, hash: word_span[index].clone(), placed_by };
                let letter_status = Letter_Status { position, status: Status::Solved };
                set!(world, (single_letter, letter_status));

                index += 1;
            }
        }
    }
}
