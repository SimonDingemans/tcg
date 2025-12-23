use std::collections::HashMap;

mod domain;
use domain::schema::CardBlueprint;

use crate::{domain::schema::BlueprintId, state::engine::GameEngine};

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

    game_engine.initialize().unwrap();

    let p1_hand = game_engine.state.players[0].hand.list();

    for (instance_id, _) in p1_hand {
        if let Some(_) = game_engine.state.entities.get(*instance_id) {
            if let Some(blueprint) = game_engine.get_blueprint(*instance_id) {
                println!("Card Name: {}", blueprint.name);
                println!("{:#?}", blueprint);
            }
        }
    }
}
