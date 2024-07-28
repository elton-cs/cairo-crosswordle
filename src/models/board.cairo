use starknet::ContractAddress;

#[derive(Copy, Drop, Serde, Debug)]
#[dojo::model]
pub struct Letter {
    #[key]
    pub position: u8,
    pub hash: felt252,
    pub placed_by: ContractAddress,
}

#[derive(Copy, Drop, Serde, Debug)]
#[dojo::model]
pub struct Word {
    #[key]
    pub player: ContractAddress,
    // pub letter: (Letter, Letter, Letter, Letter, Letter),
    pub word_hash: (felt252, felt252, felt252, felt252, felt252),
}
