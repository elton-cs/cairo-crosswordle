use dojo_starter::models::board::Letter;

#[dojo::interface]
trait ICreate {
    fn create_letter(ref world: IWorldDispatcher, letter: felt252, position: u8);
}

#[dojo::contract]
mod create_actions {
    use super::ICreate;
    use starknet::{ContractAddress, get_caller_address};
    use dojo_starter::models::board::Letter;

    #[abi(embed_v0)]
    impl CreateImpl of ICreate<ContractState> {
        fn create_letter(ref world: IWorldDispatcher, letter: felt252, position: u8) {
            let player = get_caller_address();
            let single_letter = Letter { position, player, hash: letter };

            set!(world, (single_letter));
        }
    }
}
