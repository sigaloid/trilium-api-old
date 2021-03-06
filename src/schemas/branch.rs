use nanoserde::{DeJson, SerJson};

#[derive(SerJson, DeJson)]
pub struct Branch {
    #[nserde(rename = "branchId")]
    pub branch_id: String,

    #[nserde(rename = "noteId")]
    pub note_id: String,

    #[nserde(rename = "parentNoteId")]
    pub parent_note_id: String,

    #[nserde(rename = "prefix")]
    pub prefix: String,

    #[nserde(rename = "notePosition")]
    pub note_position: usize,

    #[nserde(rename = "isExpanded")]
    pub is_expanded: bool,
    // #[nserde(rename = "utcDateModified")]
    // utc_date_modified: UtcDateModified,
}
