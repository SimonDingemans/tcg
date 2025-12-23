use serde::{Deserialize, Serialize};
use crate::domain::schema::card_types::CardType;

use crate::domain::schema::common::{BlueprintId, Environment, Rarity, ManaCost, Keyword};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardBlueprint {
    pub id: BlueprintId,
    pub name: String,
    pub description: String,
    pub domain: Environment,
    pub rarity: Rarity,
    pub card_type: CardType,

    pub cost: Vec<ManaCost>,
    pub keywords: Vec<Keyword>,
}