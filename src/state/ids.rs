use thiserror::Error;

pub type InstanceId = u16;

#[derive(Error, Debug)]
pub enum IdError {
    #[error("Ran out of possible ids")]
    Overflow
}

pub struct IdManager {
    high: InstanceId,
    is_full: bool,
    free_list: Vec<InstanceId>,
}

impl IdManager {
    pub fn new() -> Self {
        Self {
            high: 0,
            is_full: false,
            free_list: Vec::with_capacity(Self::CAPACITY)
        }
    }

    pub fn allocate(&mut self) -> Result<InstanceId, IdError> {
        if let Some(id) = self.free_list.pop() {
            return Ok(id);
        }

        if !self.is_full {
            let id = self.high;

            if id == InstanceId::MAX {
                self.is_full = true;
            } else {
                self.high += 1;
            }

            return Ok(id);
        }

        Err(IdError::Overflow)
    }

    pub fn free(&mut self, id: InstanceId) {
        debug_assert!(!self.free_list.contains(&id), "Double free detected!");

        debug_assert!(self.is_full || id < self.high, "Freed an ID that was never allocated!");

        self.free_list.push(id);
    }

    pub const CAPACITY: usize = (InstanceId::MAX as usize) + 1;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let mut manager = IdManager::new();
        
        // Should successfully allocate 0 to 255
        for i in 0..=InstanceId::MAX {
            let id = manager.allocate();
            assert!(id.is_ok(), "Failed at index {}", i);
            assert_eq!(id.unwrap(), i);
        }

        // Should fail on 256th attempt
        assert!(manager.allocate().is_err());

        // Free one and re-allocate
        manager.free(10);
        assert_eq!(manager.allocate().unwrap(), 10);
    }
}