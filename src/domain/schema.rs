use serde::{Deserialize, Serialize};
use strum::{EnumCount, FromRepr, EnumIter};

pub type BlueprintId = String;
pub type CardValue = u8;
pub type TurnValue = u8;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardType {
    Creature { power: CardValue, health: CardValue },
    Spell,
    Artifact,
    Land { resource: Environment },
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Copy, PartialEq,
    EnumCount, FromRepr, EnumIter 
)]
#[repr(u8)]
pub enum Environment {
    Forest,
    Volcano,
    Swamp,
    Plains,
    Island,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManaCost {
    Specific(Environment, CardValue),
    Neutral(CardValue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Keyword {
    Intercept,
    Stealth { duration: Duration },
    Crush,
    Aerial
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Duration {
    Turns(TurnValue),
    Permanent,
}