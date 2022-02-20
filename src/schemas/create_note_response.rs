use nanoserde::{DeJson, SerJson};

use super::{note::Note, branch::Branch};

#[derive(SerJson, DeJson)]
pub struct CreateNoteResponse {
    #[nserde(rename = "note")]
    pub note: Note,
    
    #[nserde(rename = "branch")]
    pub branch: Branch,
}