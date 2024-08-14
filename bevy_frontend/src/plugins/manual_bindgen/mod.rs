use bevy::prelude::Component;
use cainome::cairo_serde::ContractAddress;
use starknet::core::types::Felt;

#[derive(Debug, Component)]
pub struct Letter {
    pub position: u8,
    pub hash: Felt,
    pub placed_by: ContractAddress,
}

#[derive(Debug, Component)]
pub struct LetterStatus {
    pub position: u8,
    pub status: Status,
}

#[derive(Debug, Component)]
pub enum Status {
    Hidden,
    Solved,
}
