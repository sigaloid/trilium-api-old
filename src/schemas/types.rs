use std::fmt;

use nanoserde::{DeJson, SerJson};

#[derive(SerJson, DeJson, Debug, Clone)]
pub enum NoteType {
    #[nserde(rename = "text")]
    Text,

    #[nserde(rename = "code")]
    Code,

    #[nserde(rename = "render")]
    Render,

    #[nserde(rename = "file")]
    File,

    #[nserde(rename = "image")]
    Image,

    #[nserde(rename = "search")]
    Search,

    #[nserde(rename = "relation-map")]
    RelationMap,

    #[nserde(rename = "book")]
    Book,

    #[nserde(rename = "note-map")]
    NoteMap,

    #[nserde(rename = "mermaid")]
    Mermaid,
}
impl fmt::Display for NoteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
