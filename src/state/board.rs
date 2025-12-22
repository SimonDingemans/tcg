use crate::state::players::player::PlayerState;
use crate::state::{entities::EntityHeap, ids::IdManager};

pub struct GameState {
    pub entities: EntityHeap,
    pub players: [PlayerState; 2],
    pub ids: IdManager,
    pub stack: Vec<String>
}

impl GameState {
    pub fn new(
        players: [PlayerState; 2],
    ) -> Self {
        Self {
            entities: EntityHeap::new(),
            players,
            ids: IdManager::new(),
            stack: Vec::new()
        }
    }
}