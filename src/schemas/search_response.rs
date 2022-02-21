use nanoserde::{DeJson, SerJson};

use super::note::Note;

#[derive(SerJson, DeJson, Clone)]
pub struct SearchResponse {
    #[nserde(rename = "results")]
    pub results: Vec<Note>,
    // #[nserde(rename = "debugInfo")]
    // debug_info:
}
