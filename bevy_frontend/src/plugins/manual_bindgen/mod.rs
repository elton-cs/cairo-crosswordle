use bevy::prelude::Component;

#[derive(Debug, Component, Clone)]
pub struct Letter {
    pub position: u8,
    pub mock_hash: char,
    // pub hash: Felt,
    // pub placed_by: ContractAddress,
}

#[derive(Debug, Component, Clone)]
pub struct LetterStatus {
    pub position: u8,
    pub status: Status,
}

#[derive(Debug, Clone)]
pub enum Status {
    Hidden,
    Solved,
}
