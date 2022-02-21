use nanoserde::{DeJson, SerJson};

#[derive(SerJson, DeJson, Default)]
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

    pub debug: bool, // Enable debugging in response
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
