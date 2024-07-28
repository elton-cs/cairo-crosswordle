#[cfg(test)]
mod tests {
    use dojo::world::{IWorldDispatcher, IWorldDispatcherTrait};
    use dojo::utils::test::{spawn_test_world, deploy_contract};
    use dojo_starter::{
        systems::create::{create_actions, ICreateDispatcher, ICreateDispatcherTrait},
        models::board::{Letter, letter},
    };

    fn setup() -> (IWorldDispatcher, ICreateDispatcher) {
        // world setup
        let mut models = array![letter::TEST_CLASS_HASH];
        let world = spawn_test_world("dojo_starter", models);
        let contract_address = world
            .deploy_contract(
                'salt', create_actions::TEST_CLASS_HASH.try_into().unwrap(), array![].span()
            );
        let systems = ICreateDispatcher { contract_address };

        (world, systems)
    }

    #[test]
    fn test_create_single_letter() {
        let (world, systems) = setup();
        let caller = starknet::contract_address_const::<0x0>();

        // create single letter
        let character = 'N';
        let position = 1;
        systems.create_letter(character, position);

        // test
        let single_letter = get!(world, 1, (Letter));
        assert!(single_letter.position == 1, "Letter is placed in wrong position");
        assert!(single_letter.hash == 'N'.into(), "Letter is incorrect");
        assert!(single_letter.placed_by == caller, "Wrong player");
    }

    #[test]
    fn test_create_multiple_letters() {
        let (world, systems) = setup();
        let caller = starknet::contract_address_const::<0x0>();

        // create multiple letters
        let characters = ['N', 'I', 'N', 'J', 'A'].span();
        let mut index = 0;
        while (index < 5) {
            systems.create_letter(characters[index].clone(), index.try_into().unwrap());
            index += 1;
        };

        // test
        index = 0;
        while (index < 5) {
            let position: u8 = index.try_into().unwrap();
            let single_letter = get!(world, position, (Letter));
            assert!(single_letter.hash == characters[index].clone(), "Letter is incorrect");
            assert!(single_letter.placed_by == caller, "Wrong player");
            index += 1;
        };
    }

    #[test]
    fn test_word() {
        let (world, systems) = setup();

        let single_word = ['N', 'I', 'N', 'J', 'A'];
        systems.create_word(single_word);

        // test
        let mut index = 0;
        let word_span = single_word.span();
        while index < word_span
            .len() {
                let position: u8 = index.try_into().unwrap();
                let letter = get!(world, position, (Letter));
                assert_eq!(letter.hash, word_span[index].clone());
                index += 1;
            }
    }
}
