use crate::{Error, Trilium};
use nanoserde::{DeJson, DeJsonErr, SerJson};

#[derive(SerJson, DeJson)]
pub struct SearchOptions {
    pub search: String,
    #[nserde(rename = "fastSearch")]
    pub fast_search: bool, // Ignore note content
    #[nserde(rename = "includeArchivedNotes")]
    pub include_archived_notes: bool, // Include archived notes
    #[nserde(rename = "ancestorNoteId")]
    pub ancestor_note_id: Option<String>, // Search only in subtree (id)
    #[nserde(rename = "ancestorDepth")]
    pub ancestor_depth: Option<DepthOptions>, // Options for depth search
    #[nserde(rename = "orderBy")]
    pub order_by: Option<String>, // Examples include "title", "#publicationDate", "isProtected", "isArchived", "dateCreated", "dateModified", "utcDateCreated", "utcDateModified", "parentCount", "childrenCount", "attributeCount", "labelCount", "ownedLabelCount", "relationCount", "ownedRelationCount", "relationCountIncludingLinks", "ownedRelationCountIncludingLinks", "targetRelationCount", "targetRelationCountIncludingLinks", "contentSize", "noteSize", "revisionCount"
    #[nserde(rename = "orderDirection")]
    pub order_direction: Option<OrderDirection>, // Direction of ordering
    pub limit: Option<usize>, // Search limit
    pub debug: bool,          // Enable debugging in response
}
impl SearchOptions {
    pub fn default(search_query: impl ToString) -> Self {
        Self {
            search: search_query.to_string(),
            fast_search: false,
            include_archived_notes: false,
            ancestor_note_id: None,
            ancestor_depth: None,
            order_by: None,
            order_direction: None,
            limit: None,
            debug: false,
        }
    }
}
#[derive(SerJson, DeJson)]
pub enum OrderDirection {
    #[nserde(rename = "asc")]
    Ascending,
    #[nserde(rename = "dec")]
    Descending,
}
#[derive(SerJson, DeJson)]
pub enum DepthOptions {
    LessThan(usize),
    Exactly(usize),
    GreaterThan(usize),
}

#[derive(SerJson, DeJson)]
pub struct SearchResults {
    #[nserde(rename = "results")]
    pub results: Vec<SearchResult>,

    #[nserde(rename = "debugInfo")]
    pub debug_info: DebugInfo,
}

#[derive(SerJson, DeJson)]
pub struct DebugInfo {}

#[derive(SerJson, DeJson)]
pub struct SearchResult {
    #[nserde(rename = "noteId")]
    pub note_id: String,

    #[nserde(rename = "title")]
    pub title: String,

    #[nserde(rename = "type")]
    pub result_type: String,

    #[nserde(rename = "mime")]
    pub mime: String,

    #[nserde(rename = "isProtected")]
    pub is_protected: bool,

    #[nserde(rename = "attributes")]
    pub attributes: Vec<Attribute>,

    #[nserde(rename = "parentNoteIds")]
    pub parent_note_ids: Vec<String>,

    #[nserde(rename = "childNoteIds")]
    pub child_note_ids: Vec<String>,

    #[nserde(rename = "parentBranchIds")]
    pub parent_branch_ids: Vec<String>,

    #[nserde(rename = "childBranchIds")]
    pub child_branch_ids: Vec<String>,

    #[nserde(rename = "dateCreated")]
    pub date_created: String,

    #[nserde(rename = "dateModified")]
    pub date_modified: String,

    #[nserde(rename = "utcDateCreated")]
    pub utc_date_created: DebugInfo,

    #[nserde(rename = "utcDateModified")]
    pub utc_date_modified: DebugInfo,
}

#[derive(SerJson, DeJson)]
pub struct Attribute {
    #[nserde(rename = "attributeId")]
    pub attribute_id: String,

    #[nserde(rename = "noteId")]
    pub note_id: String,

    #[nserde(rename = "type")]
    pub attribute_type: String,

    #[nserde(rename = "name")]
    pub name: String,

    #[nserde(rename = "value")]
    pub value: String,

    #[nserde(rename = "position")]
    pub position: i64,

    #[nserde(rename = "isInheritable")]
    pub is_inheritable: bool,

    #[nserde(rename = "utcDateModified")]
    pub utc_date_modified: DebugInfo,
}
