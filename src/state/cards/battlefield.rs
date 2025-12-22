use thiserror::Error;

use crate::state::ids::InstanceId;

#[derive(Error, Debug)]
pub enum BattlefieldError {
    #[error("Attempted to insert a card to battlefield out of bounds at index {index:?}")]
    IndexOutOfBounds { index: usize },

    #[error("Battlefield is full when trying to add {instance_id:?}")]
    BattlefieldFull { instance_id: InstanceId },

    #[error("Attempted to remove a card not in battlefield {instance_id:?}")]
    CardNotInBattlefield { instance_id: InstanceId },
}

pub type BattlefieldList = Vec<InstanceId>;

pub struct Battlefield {
    inner: BattlefieldList,
}

impl Battlefield {
    pub fn new(size: usize) -> Self {
        Battlefield { inner: Vec::with_capacity(size) }
    }

    pub fn add(&mut self, index: usize, instance_id: InstanceId) -> Result<(), BattlefieldError> {
        if index > self.inner.len() {
            return Err(BattlefieldError::IndexOutOfBounds { index });
        }
        self.inner.insert(index, instance_id);
        Ok(())
    }

    pub fn remove(&mut self, instance_id: InstanceId) -> Result<InstanceId, BattlefieldError> {
        if let Some(pos) = self.inner.iter().position(|x| x == &instance_id) {
            Ok(self.inner.remove(pos))
        } else {
            Err(BattlefieldError::CardNotInBattlefield { instance_id })
        }
    }

    pub fn list(&self) -> &BattlefieldList {
        &self.inner
    }
}
