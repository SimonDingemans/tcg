use thiserror::Error;

use crate::state::ids::InstanceId;

#[derive(Error, Debug)]
pub enum HandError {
    #[error("Hand is full, cannot add more cards")]
    HandFull,

    #[error("Card not found in hand {instance_id:?}")]
    CardNotFound { instance_id: InstanceId },
}

pub type HandList = Vec<(InstanceId, u8)>;

pub struct Hand {
    inner: HandList,
}

impl Hand {
    pub fn new(hand_size: usize) -> Self {
        Hand { inner: Vec::with_capacity(hand_size) }
    }

    pub fn add(&mut self, instance_id: InstanceId) -> Result<&HandList, HandError> {
        if self.inner.len() >= self.inner.capacity() {
            return Err(HandError::HandFull);
        }

        if let Some(entry) = self.inner.iter_mut().find(|entry| entry.0 == instance_id) {
            entry.1 += 1;
            Ok(self.list())
        } else {
            self.inner.push((instance_id, 1));
            Ok(self.list())
        }
    }

    pub fn remove(&mut self, instance_id: InstanceId) -> Result<&HandList, HandError> {
        if let Some(index) = self.inner.iter().position(|entry| entry.0 == instance_id) {
            if self.inner[index].1 > 1 {
                self.inner[index].1 -= 1;
                Ok(self.list())
            } else {
                self.inner.remove(index);
                Ok(self.list())
            }
        } else {
            Err(HandError::CardNotFound { instance_id })
        }
    }

    pub fn list(&self) -> &HandList {
        &self.inner
    }
}