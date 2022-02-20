use nanoserde::{DeJson, SerJson};

#[derive(SerJson, DeJson)]
pub struct EntityId {
    pub id: String,
}
