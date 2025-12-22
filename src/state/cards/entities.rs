use std::collections::HashMap;

use crate::{domain::schema::{BlueprintId, CardValue}, state::ids::InstanceId, state::players::player::PlayerId};

#[derive(Clone, Debug)]
pub struct CardInstance {
    pub id: InstanceId,
    pub blueprint_id: BlueprintId,
    pub owner: PlayerId,
    pub controller: PlayerId,
    pub damage_taken: CardValue,
    pub is_tapped: bool
}

impl CardInstance {
    pub fn new(
        id: InstanceId,
        blueprint_id: BlueprintId,
        owner: PlayerId
    ) -> Self {
        Self {
            id,
            blueprint_id,
            owner: owner.clone(),
            controller: owner,
            damage_taken: 0,
            is_tapped: false
        }
    }
}

pub struct EntityHeap {
    inner: HashMap<InstanceId, CardInstance>
}

impl EntityHeap {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    pub fn register(&mut self, instance: CardInstance) {
        self.inner.insert(instance.id, instance);
    }

    pub fn get(&self, id: InstanceId) -> Option<&CardInstance> {
        self.inner.get(&id)
    }

    pub fn get_mut(&mut self, id: InstanceId) -> Option<&mut CardInstance> {
        self.inner.get_mut(&id)
    }

    pub fn contains(&self, id: InstanceId) -> bool {
        self.inner.contains_key(&id)
    }
}