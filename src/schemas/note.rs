use nanoserde::{SerJson, DeJson};

use super::{attribute::Attribute, entity_id::EntityId, types::NoteType};

#[derive(SerJson, DeJson)]
pub struct Note {
    #[nserde(rename = "noteId")]
    pub note_id: EntityId,

    #[nserde(rename = "title")]
    pub title: String,

    #[nserde(rename = "type")]
    pub note_type: NoteType,

    #[nserde(rename = "mime")]
    pub mime: String,

    #[nserde(rename = "isProtected")]
    pub is_protected: bool,

    #[nserde(rename = "attributes")]
    pub attributes: Vec<Attribute>,

    #[nserde(rename = "parentNoteIds")]
    pub parent_note_ids: Vec<EntityId>,

    #[nserde(rename = "childNoteIds")]
    pub child_note_ids: Vec<EntityId>,

    #[nserde(rename = "parentBranchIds")]
    pub parent_branch_ids: Vec<EntityId>,

    #[nserde(rename = "childBranchIds")]
    pub child_branch_ids: Vec<EntityId>,

    #[nserde(rename = "dateCreated")]
    pub date_created: String,

    #[nserde(rename = "dateModified")]
    pub date_modified: String,

    #[nserde(rename = "utcDateCreated")]
    pub utc_date_created: String,

    #[nserde(rename = "utcDateModified")]
    pub utc_date_modified: String,
}
