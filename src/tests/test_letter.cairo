#[cfg(test)]
mod tests {
    use dojo::world::{IWorldDispatcher, IWorldDispatcherTrait};
    use dojo::utils::test::{spawn_test_world, deploy_contract};
    use dojo_starter::{
        systems::create::{create_actions, ICreateDispatcher, ICreateDispatcherTrait},
        models::board::{Letter, letter},
    };

    #[test]
    fn test_create_single_letter() {
        let caller = starknet::contract_address_const::<0x0>();

        // world setup
        let mut models = array![letter::TEST_CLASS_HASH];
        let world = spawn_test_world("dojo_starter", models);
        let contract_address = world
            .deploy_contract(
                'salt', create_actions::TEST_CLASS_HASH.try_into().unwrap(), array![].span()
            );
        let actions_system = ICreateDispatcher { contract_address };

        // test function
        actions_system.create_letter('N', 1);
        let letter = get!(world, caller, (Letter));
        println!("{:?}", letter);
        assert!(letter.position == 1, "Letter is placed in wrong position");
        assert!(letter.letter == 'N'.into(), "Letter is incorrect");
    }
}
