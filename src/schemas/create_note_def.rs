use nanoserde::{DeJson, SerJson};

use super::types::NoteType;

#[derive(SerJson, DeJson)]
pub struct CreateNoteDef {
    #[nserde(rename = "parentNoteId")]
    pub parent_note_id: String,

    #[nserde(rename = "title")]
    pub title: String,

    #[nserde(rename = "type")]
    pub note_type: NoteType,

    #[nserde(rename = "mime")]
    pub mime: Option<String>,

    #[nserde(rename = "content")]
    pub content: String,

    #[nserde(rename = "notePosition")]
    pub position: Option<usize>,

    #[nserde(rename = "prefix")]
    pub prefix: Option<String>,

    #[nserde(rename = "isExpanded")]
    pub is_expanded: bool,

    #[nserde(rename = "noteId")]
    pub note_id: String,
}
