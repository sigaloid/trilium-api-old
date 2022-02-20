use nanoserde::{DeJson, SerJson};

#[derive(SerJson, DeJson)]
pub struct NoteResult {
    #[nserde(rename = "noteId")]
    pub note_id: String,

    #[nserde(rename = "title")]
    title: String,

    #[nserde(rename = "type")]
    note_type: String,

    #[nserde(rename = "mime")]
    mime: String,

    #[nserde(rename = "isProtected")]
    is_protected: bool,

    #[nserde(rename = "attributes")]
    attributes: Vec<Attribute>,

    #[nserde(rename = "parentNoteIds")]
    parent_note_ids: Vec<String>,

    #[nserde(rename = "childNoteIds")]
    child_note_ids: Vec<String>,

    #[nserde(rename = "parentBranchIds")]
    parent_branch_ids: Vec<String>,

    #[nserde(rename = "childBranchIds")]
    child_branch_ids: Vec<String>,

    #[nserde(rename = "dateCreated")]
    date_created: String,

    #[nserde(rename = "dateModified")]
    date_modified: String,
}

#[derive(SerJson, DeJson)]
pub struct Attribute {
    #[nserde(rename = "attributeId")]
    attribute_id: String,

    #[nserde(rename = "noteId")]
    note_id: String,

    #[nserde(rename = "type")]
    attribute_type: String,

    #[nserde(rename = "name")]
    name: String,

    #[nserde(rename = "value")]
    value: String,

    #[nserde(rename = "position")]
    position: i64,

    #[nserde(rename = "isInheritable")]
    is_inheritable: bool,
}
