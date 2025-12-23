use thiserror::Error;

use crate::state::cards::entities::EntityHeap;
use crate::state::players::player::{PlayerId, PlayerState};
use crate::state::ids::IdManager;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Transition error: {:?}", status)]
    Transition { status: GameStatus }
}

#[derive(Debug)]
pub enum GameStatus {
    Initializing,
    Active,
    Finished(GameResult)
}

#[derive(Debug)]
pub enum GameResult {
    HealthZero(PlayerId),
    DeckedOut(PlayerId),
    Conceded(PlayerId)
}

pub struct Turn {
    pub current_player: usize,
    pub number: usize
}

pub struct GameState {
    pub entities: EntityHeap,
    pub players: [PlayerState; 2],
    pub ids: IdManager,
    pub stack: Vec<String>,
    status: GameStatus,
    turn: Turn
}

impl GameState {
    pub fn new(
        players: [PlayerState; 2],
    ) -> Self {
        Self {
            entities: EntityHeap::new(),
            players,
            ids: IdManager::new(),
            stack: Vec::new(),
            status: GameStatus::Initializing,
            turn: Turn {
                current_player: 0,
                number: 0
            }
        }
    }

    pub fn transition(&mut self, result: Option<GameResult>) -> Result<(), GameError> {
        match self.status {
            GameStatus::Initializing => {
                self.status = GameStatus::Active;
                Ok(())
            },
            GameStatus::Active => {
                if let Some(result) = result {
                    self.status = GameStatus::Finished(result);
                    Ok(())
                } else {
                    Err(GameError::Transition { status: GameStatus::Active })
                }
            },
            GameStatus::Finished(_) => Ok(())
        }
    }

    pub fn next_turn(&mut self) -> &Turn {
        self.turn.current_player = (self.turn.current_player + 1) % self.players.len();
        self.turn.number += 1;
        &self.turn
    }

    pub fn turn(&self) -> &Turn {
        &self.turn
    }

    pub fn status(&self) -> &GameStatus {
        &self.status
    }
}