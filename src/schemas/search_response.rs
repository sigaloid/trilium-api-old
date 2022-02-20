use nanoserde::{SerJson, DeJson};

use super::{note::Note};

#[derive(SerJson, DeJson)]
pub struct SearchResponse {
    #[nserde(rename = "results")]
    pub results: Vec<Note>,

    // #[nserde(rename = "debugInfo")]
    // debug_info: 
}