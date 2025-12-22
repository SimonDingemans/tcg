use std::collections::HashMap;

use strum::EnumCount;
use thiserror::Error;

use crate::{domain::schema::{BlueprintId, CardBlueprint, CardValue, Environment}, state::{cards::{battlefield::{Battlefield, BattlefieldError}, deck::{Deck, DeckList}, graveyard::Graveyard, hand::{Hand, HandList}}, ids::InstanceId, players::resources::ManaPool}};

pub type PlayerId = String;

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("Battlefield was full when trying to add {instance_id:?}")]
    BattlefieldFull { instance_id: InstanceId },

    #[error("Attempted to insert a card to battlefield out of bounds at index {index:?}")]
    BattlefieldIndexOutOfBounds { index: usize },

    #[error("Attempted to play a card not in hand {instance_id:?}")]
    CardNotInHand { instance_id: InstanceId }

}

pub struct PlayerState {
    pub id: PlayerId,
    pub health: usize,
    pub mana: ManaPool,
    pub deck: Deck,
    pub hand: Hand,
    pub battlefield: Battlefield,
    pub graveyard: Graveyard
}

impl PlayerState {
    pub fn new(
        id: PlayerId,
        health: usize,
        deck: Deck,
        hand: Hand,
        battlefield_size: usize,
        graveyard_size: usize,
    ) -> Self {
        let mana: [CardValue; Environment::COUNT] = [0; Environment::COUNT];
        let mana_pool = ManaPool { mana };

        Self { 
            id, 
            health,
            mana: mana_pool, 
            deck, 
            hand, 
            battlefield: Battlefield::new(battlefield_size), 
            graveyard: Graveyard::new(graveyard_size) 
        }
    }

    pub fn add_to_battlefield(&mut self, index: usize, instance_id: InstanceId ) -> Result<(), BattlefieldError> {
        self.battlefield.add(index, instance_id)
    }

    pub fn remove_from_battlefield(&mut self, instance_id: InstanceId) -> Result<InstanceId, BattlefieldError> {
        self.battlefield.remove(instance_id)
    }
}