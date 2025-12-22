use std::collections::HashMap;

mod domain;
use domain::schema::CardBlueprint;

use crate::{domain::schema::BlueprintId, state::engine::{GameEngine}};

mod state;

fn main() {
    let blueprint_sources = [
        include_str!("../blueprints/forest_set.ron"),
        include_str!("../blueprints/volcanic_set.ron"),
    ];

    let mut blueprints: HashMap<BlueprintId, CardBlueprint> = HashMap::new();
    for source in &blueprint_sources {
        match ron::from_str::<Vec<CardBlueprint>>(source) {
            Ok(cards) => {
                for card in cards {
                    blueprints.insert(card.id.clone(), card);
                }
            }
            Err(e) => {
                eprintln!("Failed to parse card database: {}", e);
                return;
            }
        }
    }

    let player_ids = ["Player".to_owned(), "AI".to_owned()];


    let mut game_engine = GameEngine::new(blueprints, player_ids);

    let instance_id = match game_engine.spawn_instance("forest_set_003".to_owned(), "Player".to_owned()) {
        Some(id) => id,
        None => return
    };
    
    game_engine.add_to_battlefield(instance_id, "Player".to_owned(), 0);

    let battlefield_ids = game_engine.state.players[0].battlefield.list();

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
