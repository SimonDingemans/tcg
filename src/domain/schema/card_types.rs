use serde::{Deserialize, Serialize};

use crate::domain::schema::common::{CardValue, Environment, Keyword, Target, Duration, ManaCost};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardType {
    Creature { power: CardValue, health: CardValue },
    Spell { action: SpellAction },
    Artifact { kind: ArtifactType },
    Land { resource: Environment, amount: CardValue },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpellAction {
    Damage { target: Target, amount: CardValue },
    Heal { target: Target, amount: CardValue },
    ModifyStats { 
        target: Target, 
        power: CardValue, 
        health: CardValue, 
        duration: Duration 
    },
    GrantKeyword { 
        target: Target, 
        keyword: Keyword, 
        duration: Duration 
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactType {
    Static { effect: StaticEffect },
    Activated { cost: Vec<ManaCost>, action: SpellAction },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StaticEffect {
    GlobalStatBoost { target: Target, power: CardValue, health: CardValue },
}