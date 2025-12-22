use std::collections::HashMap;

use crate::{domain::schema::{BlueprintId, CardBlueprint, Environment}, state::{board::GameState, cards::{deck::Deck, hand::Hand}, entities::CardInstance, ids::InstanceId, players::player::{PlayerId, PlayerState}}};

pub struct GameEngine {
    pub state: GameState,
    pub blueprints: HashMap<BlueprintId, CardBlueprint>
}

impl GameEngine {
    pub const BATTLE_FIELD_SIZE: usize = 9;
    pub const STARTING_HAND_SIZE: usize = 5;

    pub fn new(
        blueprints: HashMap<BlueprintId, CardBlueprint>,
        player_ids: [PlayerId; 2]
    ) -> Self {
        let forest_deck = Deck::new(&blueprints, Environment::Forest);
        let volcanic_deck = Deck::new(&blueprints, Environment::Volcano);

        let players = [
            PlayerState::new(
                player_ids[0].clone(),
                forest_deck,
                Hand::new(Self::STARTING_HAND_SIZE),
                Self::BATTLE_FIELD_SIZE,
                50
            ),
            PlayerState::new(
                player_ids[1].clone(),
                volcanic_deck,
                Hand::new(Self::STARTING_HAND_SIZE),
                Self::BATTLE_FIELD_SIZE,
                50
            )
        ];

        Self {
            state: GameState::new(players),
            blueprints
        }
    }

    pub fn spawn_instance(&mut self, blueprint_id: BlueprintId, owner: PlayerId) -> Option<InstanceId> {
        if !self.blueprints.contains_key(&blueprint_id) {
            return None;
        }

        let card_instance = match self.state.ids.allocate() {
            Ok(id) => CardInstance::new(id, blueprint_id, owner),
            Err(_) => return None
        };

        let id = card_instance.id;

        self.state.entities.register(card_instance);

        Some(id)
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