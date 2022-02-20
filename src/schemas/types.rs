use std::fmt;

use nanoserde::{DeJson, SerJson};

#[derive(SerJson, DeJson, Debug)]
pub enum NoteType {
    #[nserde(rename = "text")]
    Text,

    #[nserde(rename = "code")]
    Code,

    #[nserde(rename = "file")]
    File,

    #[nserde(rename = "image")]
    Image,

    #[nserde(rename = "search")]
    Search,

    #[nserde(rename = "book")]
    Book,

    #[nserde(rename = "relationmap")]
    Relationmap,

    #[nserde(rename = "render")]
    Render,
}
impl fmt::Display for NoteType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
