use crate::domain::schema::{CardBlueprint, ManaCost, card_types::CardType};


impl CardBlueprint {
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        self.validate_id(&mut errors);
        self.validate_name(&mut errors);
        self.validate_description(&mut errors);
        self.validate_cost(&mut errors);
        errors.extend(self.card_type.validate());

        errors
    }
}

impl CardBlueprint {
    fn validate_id(&self, errors: &mut Vec<String>) {
        if self.id.is_empty() {
            errors.push("CardBlueprint ID cannot be empty.".to_string());
        } else if let Some((set_name, number_str)) = self.id.rsplit_once("_set_") {
            if set_name.is_empty() {
                errors.push("ID format error: Set name cannot be empty (must appear before '_set_').".to_string());
            }
            if number_str.len() != 3 {
                errors.push("ID format error: Number suffix must be exactly 3 digits.".to_string());
            }
            if number_str.parse::<u16>().is_err() {
                errors.push(format!("ID format error: Suffix '{}' is not a valid number.", number_str));
            }
        } else {
            errors.push("ID format error: Must contain '_set_' separating set name and number.".to_string());
        }
    }

    fn validate_name(&self, errors: &mut Vec<String>) {
        if self.name.trim().is_empty() {
            errors.push("CardBlueprint name cannot be empty.".to_string());
        }
    }

    fn validate_description(&self, errors: &mut Vec<String>) {
        if self.description.trim().is_empty() {
            errors.push("CardBlueprint description cannot be empty.".to_string());
        }
    }

    fn validate_cost(&self, errors: &mut Vec<String>) {
        for cost in &self.cost {
            match cost {
                ManaCost::Specific(env, ..) => {
                    if env != &self.domain {
                        errors.push(format!(
                            "ManaCost error: Specific mana cost environment '{:?}' does not match card domain '{:?}'.",
                            env, self.domain
                        ));
                    }
                }
                ManaCost::Neutral(_) => { /* Neutral costs are always valid */ }
            }
        }
    }
}

impl CardType {
    fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        match self {
            CardType::Creature { health, .. } => {
                if *health == 0 {
                    errors.push("Creature health must be greater than 0.".to_string());
                }
            }
            CardType::Land { amount, .. } => {
                if *amount == 0 {
                    errors.push("Land amount must be greater than 0.".to_string());
                }
            }
            _ => {}
        }

        errors
    }
}