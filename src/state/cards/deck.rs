use std::collections::HashMap;

use rand::seq::SliceRandom;
use thiserror::Error;

use crate::{domain::schema::{BlueprintId, CardBlueprint, CardType, Environment}, state::cards};

#[derive(Error, Debug)]
pub enum DeckError {
    #[error("Deck is empty, cannot draw more cards")]
    EmptyDeck,
}

pub type DeckList = Vec<(BlueprintId, u8)>;

pub struct Deck {
    inner: DeckList,
}

impl Deck {
    pub fn new(blueprints: &HashMap<BlueprintId, CardBlueprint>, domain: Environment) -> Self {
        let mut deck = Vec::new();

        blueprints.iter().filter(|blueprint| blueprint.1.domain == domain).for_each(|(id, blueprint)| {
            let count = match blueprint.card_type {
                CardType::Land { .. } => 8,
                CardType::Creature { .. } => 4,
                CardType::Spell => 4,
                CardType::Artifact => 2,
            };
            deck.push((id.clone(), count));
        });

        Deck { inner: deck }
    }

    pub fn list(&self) -> &DeckList {
        &self.inner
    }
    
    pub fn shuffle(&mut self) -> &DeckList{
        let mut rng = rand::rng();
        let _ = &self.inner.shuffle(&mut rng);
        self.list()
    }

    pub fn draw(&mut self) -> Result<BlueprintId, DeckError> {
        match self.inner.pop() {
            Some((blueprint_id, mut count)) => {
                if count > 1 {
                    count -= 1;
                    self.inner.insert(0, (blueprint_id.clone(), count));
                    Ok(blueprint_id)
                } else {
                    Ok(blueprint_id)
                }
            } 
            None => Err(DeckError::EmptyDeck)
        }
    }
}