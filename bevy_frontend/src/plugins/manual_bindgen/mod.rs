use cainome::cairo_serde::ContractAddress;
use dojo_types::schema::Struct as DojoStruct;
use starknet::core::types::Felt;

pub trait ToriiToBevy<T> {
    fn dojo_model_to_bevy_component(model: &DojoStruct) -> T;
}

pub struct Letter {
    pub position: u8,
    pub hash: starknet::core::types::Felt,
    pub placed_by: cainome::cairo_serde::ContractAddress,
}

impl ToriiToBevy<Letter> for Letter {
    fn dojo_model_to_bevy_component(model: &DojoStruct) -> Letter {
        let position: u8;
        position = model.children[0]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();

        let hash: Felt;
        hash = model.children[1]
            .ty
            .as_primitive()
            .unwrap()
            .as_felt252()
            .unwrap();

        let placed_by: ContractAddress;
        placed_by = ContractAddress::from(
            model.children[2]
                .ty
                .as_primitive()
                .unwrap()
                .as_contract_address()
                .unwrap(),
        );

        Letter {
            position,
            hash,
            placed_by,
        }
    }
}
