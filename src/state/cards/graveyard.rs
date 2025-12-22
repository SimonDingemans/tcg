use crate::domain::schema::BlueprintId;

pub type GraveyardList = Vec<(BlueprintId, u8)>;

pub struct Graveyard {
    inner: GraveyardList,
}

impl Graveyard {
    pub fn new(size: usize) -> Self {
        Graveyard { inner: Vec::with_capacity(size) }
    }
}
