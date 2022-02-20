use crate::{Error, Trilium};
use nanoserde::{DeJson, DeJsonErr, SerJson};

#[derive(SerJson, DeJson)]
pub struct SearchOptions {
    search: String,
    #[nserde(rename = "fastSearch")]
    fast_search: bool, // Ignore note content
    #[nserde(rename = "includeArchivedNotes")]
    include_archived_notes: bool, // Include archived notes
    #[nserde(rename = "ancestorNoteId")]
    ancestor_note_id: Option<String>, // Search only in subtree (id)
    #[nserde(rename = "ancestorDepth")]
    ancestor_depth: Option<DepthOptions>, // Options for depth search
    #[nserde(rename = "orderBy")]
    order_by: Option<String>, // Examples include "title", "#publicationDate", "isProtected", "isArchived", "dateCreated", "dateModified", "utcDateCreated", "utcDateModified", "parentCount", "childrenCount", "attributeCount", "labelCount", "ownedLabelCount", "relationCount", "ownedRelationCount", "relationCountIncludingLinks", "ownedRelationCountIncludingLinks", "targetRelationCount", "targetRelationCountIncludingLinks", "contentSize", "noteSize", "revisionCount"
    #[nserde(rename = "orderDirection")]
    order_direction: Option<OrderDirection>, // Direction of ordering
    limit: Option<usize>, // Search limit
    debug: bool,          // Enable debugging in response
}
impl SearchOptions {
    fn default(search_query: impl ToString) -> Self {
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
enum OrderDirection {
    #[nserde(rename = "asc")]
    Ascending,
    #[nserde(rename = "dec")]
    Descending,
}
#[derive(SerJson, DeJson)]
enum DepthOptions {
    LessThan(usize),
    Exactly(usize),
    GreaterThan(usize),
}

pub fn search_notes(
    trilium: &Trilium,
    search_options: SearchOptions,
) -> Result<SearchResults, crate::Error> {
    let req = trilium
        .agent
        .post(&format!("{}/etapi/notes", trilium.url))
        .send_string(&search_options.serialize_json());
    match req {
        Ok(response) => {
            if let Ok(string) = response.into_string() {
                let parse: Result<SearchResults, DeJsonErr> =
                    nanoserde::DeJson::deserialize_json(&string);
                if let Ok(note_response) = parse {
                    Ok(note_response)
                } else {
                    Err(Error::InvalidServerResponse(Some(string)))
                }
            } else {
                Err(Error::InvalidServerResponse(None))
            }
        }
        Err(e) => match e {
            ureq::Error::Status(_, _) => Err(Error::WrongPassword),
            ureq::Error::Transport(_) => Err(Error::InvalidUrl),
        },
    }
}

#[derive(SerJson, DeJson)]
pub struct SearchResults {
    #[nserde(rename = "results")]
    results: Vec<SearchResult>,

    #[nserde(rename = "debugInfo")]
    debug_info: DebugInfo,
}

#[derive(SerJson, DeJson)]
pub struct DebugInfo {}

#[derive(SerJson, DeJson)]
pub struct SearchResult {
    #[nserde(rename = "noteId")]
    note_id: String,

    #[nserde(rename = "title")]
    title: String,

    #[nserde(rename = "type")]
    result_type: String,

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
    utc_date_created: DebugInfo,

    #[nserde(rename = "utcDateModified")]
    utc_date_modified: DebugInfo,
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
    utc_date_modified: DebugInfo,
}
