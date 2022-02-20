use nanoserde::{DeJson, SerJson};

#[derive(SerJson, DeJson)]
pub struct Note {
    #[nserde(rename = "parentNoteId")]
    parent_note_id: String,

    #[nserde(rename = "title")]
    title: String,

    #[nserde(rename = "type")]
    welcome2_type: String,

    #[nserde(rename = "mime")]
    mime: String,

    #[nserde(rename = "content")]
    content: String,

    #[nserde(rename = "notePosition")]
    note_position: i64,

    #[nserde(rename = "prefix")]
    prefix: String,

    #[nserde(rename = "isExpanded")]
    is_expanded: bool,

    #[nserde(rename = "noteId")]
    note_id: String,

    #[nserde(rename = "branchId")]
    branch_id: String,
}

#[derive(SerJson, DeJson)]
pub struct NoteResponse {
    #[nserde(rename = "note")]
    note: Note,

    #[nserde(rename = "branch")]
    branch: Branch,
}

#[derive(SerJson, DeJson)]
pub struct Branch {
    #[nserde(rename = "branchId")]
    branch_id: String,

    #[nserde(rename = "noteId")]
    note_id: String,

    #[nserde(rename = "parentNoteId")]
    parent_note_id: String,

    #[nserde(rename = "prefix")]
    prefix: String,

    #[nserde(rename = "notePosition")]
    note_position: i64,

    #[nserde(rename = "isExpanded")]
    is_expanded: bool,

    #[nserde(rename = "utcDateModified")]
    utc_date_modified: UtcDate,
}

#[derive(SerJson, DeJson)]
pub struct UtcDate {}

#[derive(SerJson, DeJson)]
pub struct NoteFromServer {
    #[nserde(rename = "noteId")]
    note_id: String,

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

    #[nserde(rename = "utcDateCreated")]
    utc_date_created: UtcDate,

    #[nserde(rename = "utcDateModified")]
    utc_date_modified: UtcDate,
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

    #[nserde(rename = "utcDateModified")]
    utc_date_modified: UtcDate,
}
