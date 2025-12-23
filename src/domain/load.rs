use std::collections::{HashMap, HashSet};

use crate::domain::schema::{CardBlueprint, BlueprintId};

pub fn load_blueprints(sources: &[&str]) -> Result<HashMap<BlueprintId, CardBlueprint>, Vec<String>> {
    let mut errors = Vec::new();
    let mut all_blueprints = Vec::new();

    for source in sources {
        match ron::from_str::<Vec<CardBlueprint>>(source) {
            Ok(cards) => {
                all_blueprints.extend(cards);
            }
            Err(e) => {
                errors.push(format!("Failed to parse RON source: {}", e));
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let mut blueprint_ids: HashSet<BlueprintId> = std::collections::HashSet::new();
    let mut blueprint_names: HashSet<String> = std::collections::HashSet::new();

    for blueprint in &all_blueprints {
        if !blueprint_ids.insert(blueprint.id.clone()) {
            errors.push(format!("Duplicate Blueprint ID found: {}", blueprint.id));
        }
        if !blueprint_names.insert(blueprint.name.clone()) {
            errors.push(format!("Duplicate Blueprint name found: {}", blueprint.name));
        }
        errors.extend(blueprint.validate());
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let blueprints_map = all_blueprints
        .into_iter()
        .map(|bp| (bp.id.clone(), bp))
        .collect();

    Ok(blueprints_map)
}