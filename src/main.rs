use crate::state::engine::GameEngine;
use crate::domain::load::load_blueprints;

mod domain;
mod state;

fn main() {
    let blueprint_sources = [
        include_str!("../blueprints/forest_set.ron"),
        include_str!("../blueprints/volcanic_set.ron"),
    ];

    let blueprints = match load_blueprints(&blueprint_sources) {
        Ok(bp) => bp,
        Err(errors) => {
            eprintln!("Failed to load card blueprints:");
            for error in errors {
                eprintln!("- {}", error);
            }
            return;
        }
    };

    let player_ids = ["Player".to_owned(), "AI".to_owned()];

    let mut game_engine = GameEngine::new(blueprints, player_ids);

    game_engine.initialize().unwrap();

    let p1_hand = game_engine.state.players[0].hand.list();

    for (instance_id, _) in p1_hand {
        if game_engine.state.entities.get(*instance_id).is_some()
            && let Some(blueprint) = game_engine.get_blueprint(*instance_id) {
                println!("Card Name: {}", blueprint.name);
                println!("{:#?}", blueprint);
            }
    }
}

