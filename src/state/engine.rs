use std::collections::HashMap;

use crate::{domain::schema::{BlueprintId, CardBlueprint}, state::{board::{GameState, PlayerId}, entities::CardInstance, ids::InstanceId}};

pub struct GameEngine {
    pub state: GameState,
    pub blueprints: HashMap<BlueprintId, CardBlueprint>
}

impl GameEngine {
    pub fn new(
        blueprints: HashMap<BlueprintId, CardBlueprint>,
        player_ids: [PlayerId; 2]
    ) -> Self {
        Self {
            state: GameState::new(player_ids),
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