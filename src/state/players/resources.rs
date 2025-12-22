use std::ops::{Index, IndexMut};

use strum::EnumCount;
use thiserror::Error;

use crate::domain::schema::{CardValue, Environment};

#[derive(Error, Debug)]
pub enum ManaError {
    #[error("Inssufficient {env:?} mana: needed {needed}, available {available}")]
    InsufficientMana { env: Environment, needed: u8, available: u8 }
}

pub struct ManaPool {
    pub mana: [CardValue; Environment::COUNT]
}

impl ManaPool {
    pub fn new() -> Self {
        Self {
            mana: [0; Environment::COUNT]
        }
    }

    pub fn add(&mut self, env: Environment, amount: u8) {
        self[env] = self[env].saturating_add(amount);
    }

    pub fn get(&self, env: Environment) -> u8 {
        self[env]
    }

    pub fn total_available(&self) -> u16 {
        let mut total: u16 = 0;
        for env in self.mana {
            total += env as u16;
        }
        total
    }

    pub fn try_pay_exact(&mut self, env: Environment, amount: u8) -> Result<(), ManaError> {
        if self[env] >= amount {
            self[env] -= amount;
            Ok(())
        } else {
            Err(ManaError::InsufficientMana { env, needed: amount, available: self[env] })
        }
    }
}

impl Index<Environment> for ManaPool {
    type Output = u8;

    fn index(&self, env: Environment) -> &Self::Output {
        &self.mana[env as usize]
    }
}

impl IndexMut<Environment> for ManaPool {
    fn index_mut(&mut self, env: Environment) -> &mut Self::Output {
        &mut self.mana[env as usize]
    }
}