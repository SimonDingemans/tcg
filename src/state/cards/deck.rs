use std::collections::HashMap;

use crate::domain::schema::{BlueprintId, CardBlueprint, CardType, Environment};

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
}