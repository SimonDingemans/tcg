use std::collections::HashMap;
use std::cmp::min;

use rand::seq::SliceRandom;
use thiserror::Error;

use crate::domain::schema::{BlueprintId, CardBlueprint, Environment, Rarity};
 use crate::domain::schema::card_types::{CardType};

#[derive(Error, Debug)]
pub enum DeckError {
    #[error("Deck is empty, cannot draw more cards")]
    EmptyDeck,
}

pub struct Deck {
    inner: Vec<BlueprintId>,
}

impl Deck {
    pub fn new(blueprints: &HashMap<BlueprintId, CardBlueprint>, domain: Environment) -> Self {
        let mut deck_list = Vec::new();

        blueprints.iter().filter(|blueprint| blueprint.1.domain == domain).for_each(|(id, blueprint)| {
            let base_count = match blueprint.card_type {
                CardType::Land { .. } => 8,
                CardType::Creature { .. } => 6,
                CardType::Spell { .. } => 4,
                CardType::Artifact { .. } => 2,
            };

            let rarity_cap = match blueprint.rarity {
                Rarity::Common => base_count, // No cap
                Rarity::Uncommon => 3,
                Rarity::Rare => 2,
                Rarity::Epic => 1,
                Rarity::Legendary => 1,
            };

            let count = min(base_count, rarity_cap);

            for _ in 0..count {
                deck_list.push(id.clone());
            }
        });

        Deck { inner: deck_list }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.inner.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Result<BlueprintId, DeckError> {
        self.inner.pop().ok_or(DeckError::EmptyDeck)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}