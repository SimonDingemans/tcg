use serde::{Deserialize, Serialize};
use strum::{EnumCount, FromRepr, EnumIter};

pub type BlueprintId = String;
pub type CardValue = u8;
pub type TurnValue = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
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
pub enum Target {
    Player,
    Creature,
    AllCreatures,
    AlliedCreatures,
    EnemyCreatures,
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
    Aerial,
    Quick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Duration {
    Turns(TurnValue),
    Permanent,
}