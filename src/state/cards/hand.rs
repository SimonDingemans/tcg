use crate::state::{cards::deck::Deck, ids::InstanceId};

pub type HandList = Vec<(InstanceId, u8)>;

pub struct Hand {
    inner: HandList,
}

impl Hand {
    pub fn new(hand_size: usize) -> Self {
        Hand { inner: Vec::with_capacity(hand_size) }
    }
}