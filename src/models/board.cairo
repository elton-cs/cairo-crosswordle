use starknet::ContractAddress;

#[derive(Copy, Drop, Serde, Debug)]
#[dojo::model]
pub struct Letter {
    #[key]
    pub position: u8,
    pub hash: felt252,
    pub placed_by: ContractAddress,
}

#[derive(Copy, Drop, Serde, Debug, PartialEq, Introspect)]
pub enum Status {
    Hidden,
    Solved,
}

impl StatusFelt252 of Into<Status, felt252> {
    fn into(self: Status) -> felt252 {
        match self {
            Status::Hidden => 0,
            Status::Solved => 1,
        }
    }
}

#[derive(Copy, Drop, Serde, Debug)]
#[dojo::model]
pub struct Letter_Status {
    #[key]
    pub position: u8,
    pub status: Status,
}
