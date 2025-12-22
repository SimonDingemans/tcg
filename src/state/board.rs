use rand::{rng, seq::SliceRandom};
use strum::EnumCount;
use thiserror::Error;

use crate::{domain::schema::{CardValue, Environment}, state::{entities::{EntityHeap}, ids::{IdManager, InstanceId}, resources::ManaPool}};

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
    pub mana: ManaPool,
    pub deck: Vec<InstanceId>,
    pub hand: Vec<InstanceId>,

    pub battlefield: Vec<InstanceId>,
    pub graveyard: Vec<InstanceId>
}

impl PlayerState {
    pub fn new(
        id: PlayerId,
        mut deck: Vec<InstanceId>
    ) -> Self {
        let starting_hand_size: usize = 5;

        deck.push(0);

        let mut rng = rng();
        deck.shuffle(&mut rng);

        let split_index = deck.len().saturating_sub(starting_hand_size);
        let hand = deck.split_off(split_index);

        let mana: [CardValue; Environment::COUNT] = [0; Environment::COUNT];
        let mana_pool = ManaPool { mana };

        Self { 
            id, 
            mana: mana_pool, 
            deck, 
            hand, 
            battlefield: Vec::with_capacity(Self::BATTLE_FIELD_SIZE), 
            graveyard: Vec::with_capacity(IdManager::CAPACITY) 
        }
    }

    pub fn add_to_battlefield(&mut self, index: usize, instance_id: InstanceId) -> Result<(), PlayerError> {
        if self.battlefield.len() >= Self::BATTLE_FIELD_SIZE {
            return Err(PlayerError::BattlefieldFull { instance_id });
        }

        if index > self.battlefield.len() {
            return Err(PlayerError::BattlefieldIndexOutOfBounds { index });
        }

        if let Some(hand_index) = self.hand.iter().position(|&c| c == instance_id) {
            self.hand.remove(hand_index);
        } else {
            return Err(PlayerError::CardNotInHand { instance_id });
        }

        self.battlefield.insert(index, instance_id);

        Ok(())
    }

    pub fn remove_from_battlefield(&mut self, index: usize) -> InstanceId {
        self.battlefield.remove(index)
    }

    pub const BATTLE_FIELD_SIZE: usize = 9;
}

pub struct GameState {
    pub entities: EntityHeap,
    pub players: [PlayerState; 2],
    pub ids: IdManager,
    pub stack: Vec<String>
}

impl GameState {
    pub fn new(
        player_ids: [PlayerId; 2]
    ) -> Self {
        let players = player_ids.map(|id| {
            PlayerState::new(id, Vec::new())
        });

        Self {
            entities: EntityHeap::new(),
            players,
            ids: IdManager::new(),
            stack: Vec::new()
        }
    }
}