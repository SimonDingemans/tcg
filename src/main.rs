use std::collections::HashMap;

mod domain;
use domain::schema::CardBlueprint;

use crate::{domain::schema::BlueprintId, state::engine::{GameEngine}};

mod state;

fn main() {
    let raw_data = include_str!("../blueprints/core_set.ron");

    let blueprints: HashMap<BlueprintId, CardBlueprint> = match ron::from_str::<Vec<CardBlueprint>>(raw_data) {
        Ok(cards) => cards
            .into_iter()
            .map(|card| (card.id.clone(), card))
            .collect(),
        Err(e) => {
            eprintln!("Failed to parse card database: {}", e);
            return;
        }
    };

    let player_ids = ["Player".to_owned(), "AI".to_owned()];


    let mut game_engine = GameEngine::new(blueprints, player_ids);

    let instance_id = match game_engine.spawn_instance("core_set_003".to_owned(), "Player".to_owned()) {
        Some(id) => id,
        None => return
    };
    
    game_engine.add_to_battlefield(instance_id, "Player".to_owned(), 0);

    let battlefield_ids = &game_engine.state.players[0].battlefield.clone();

    println!("--- Battlefield Cards ---");
    for id in battlefield_ids {
        if let Some(blueprint) = game_engine.get_blueprint(*id) {
            println!("Found Card: {}", blueprint.name);
            println!("{:#?}", blueprint);
        } else {
            println!("Could not resolve blueprint for Instance ID: {:?}", id);
        }
    }
}
