use nanoserde::{DeJson, SerJson};

use super::{branch::Branch, note::Note};

#[derive(SerJson, DeJson)]
pub struct CreateNoteResponse {
    #[nserde(rename = "note")]
    pub note: Note,

    #[nserde(rename = "branch")]
    pub branch: Branch,
}
