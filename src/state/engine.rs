use std::collections::HashMap;

use thiserror::Error;

use crate::{domain::schema::{BlueprintId, CardBlueprint, Environment}, state::{board::GameState, cards::{deck::{Deck, DeckError}, entities::CardInstance, hand::Hand}, ids::{IdError, IdManager, InstanceId}, players::player::{PlayerId, PlayerState}}};

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Deck error: {0}")]
    Deck(#[from] DeckError),
    
    #[error("ID error: {0}")]
    Id(#[from] IdError),
}

pub struct GameEngine {
    pub state: GameState,
    pub blueprints: HashMap<BlueprintId, CardBlueprint>
}

impl GameEngine {
    pub const BATTLE_FIELD_SIZE: usize = 9;
    pub const MAX_HAND_SIZE: usize = 9;
    pub const STARTING_HEALTH: usize = 50;
    pub const INITIAL_HAND_SIZE: usize = 5;

    pub fn new(
        blueprints: HashMap<BlueprintId, CardBlueprint>,
        player_ids: [PlayerId; 2]
    ) -> Self {
        let forest_deck = Deck::new(&blueprints, Environment::Forest);
        let volcanic_deck = Deck::new(&blueprints, Environment::Volcano);

        let players = [
            PlayerState::new(
                player_ids[0].clone(),
                Self::STARTING_HEALTH,
                forest_deck,
                Hand::new(Self::MAX_HAND_SIZE),
                Self::BATTLE_FIELD_SIZE,
                IdManager::CAPACITY
            ),
            PlayerState::new(
                player_ids[1].clone(),
                Self::STARTING_HEALTH,
                volcanic_deck,
                Hand::new(Self::MAX_HAND_SIZE),
                Self::BATTLE_FIELD_SIZE,
                IdManager::CAPACITY
            )
        ];
        
        Self {
            state: GameState::new(players),
            blueprints
        }
    }

    pub fn initialize(&mut self) -> Result<(), GameError> {
        let player_ids: Vec<_> = self.state.players.iter().map(|p| p.id.clone()).collect();

        for player_id in player_ids {
            let blueprints_to_draw = {
                let player = self.state.players.iter_mut().find(|p| p.id == player_id).unwrap();
                player.deck.shuffle();
                let mut blueprints = Vec::new();
                for _ in 0..Self::INITIAL_HAND_SIZE {
                    match player.deck.draw() {
                        Ok(id) => blueprints.push(id),
                        Err(e) => return Err(GameError::Deck(e)),
                    }
                }
                blueprints
            };

            let mut new_instances = Vec::new();
            for blueprint_id in blueprints_to_draw {
                let instance_id = self.spawn_instance(blueprint_id, player_id.clone())?;
                new_instances.push(instance_id);
            }

            let player = self.state.players.iter_mut().find(|p| p.id == player_id).unwrap();
            for instance_id in new_instances {
                let _ = player.hand.add(instance_id);
            }
        }

        Ok(())
    }

    pub fn spawn_instance(&mut self, blueprint_id: BlueprintId, owner: PlayerId) -> Result<InstanceId, GameError> {
        let card_instance = match self.state.ids.allocate() {
            Ok(id) => CardInstance::new(id, blueprint_id, owner),
            Err(err) => return Err(GameError::Id(err))
        };

        let id = card_instance.id;

        self.state.entities.register(card_instance);

        Ok(id)
    }

    pub fn add_to_battlefield(&mut self, instance_id: InstanceId, player_id: PlayerId, index: usize) {
        let player_state = match self.state.players.iter_mut().find(|p| p.id == player_id) {
            Some(player) => player,
            None => return,
        };

        match player_state.add_to_battlefield(index, instance_id) {
            Ok(_) => (),
            Err(err) => eprintln!("{}", err)
        }
    }

    pub fn get_blueprint(&self, instance_id: InstanceId) -> Option<&CardBlueprint> {
        let instance = self.state.entities.get(instance_id)?;
        self.blueprints.get(&instance.blueprint_id)
    }
}