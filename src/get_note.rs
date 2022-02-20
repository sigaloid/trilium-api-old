use crate::{Error, Trilium};
use nanoserde::{DeJson, DeJsonErr, SerJson};

pub fn get_note(trilium: &Trilium, id: impl ToString) -> Result<NoteResult, crate::Error> {
    let req = trilium
        .agent
        .get(&format!("{}/etapi/notes/{}", trilium.url, id.to_string()))
        .call();
    match req {
        Ok(response) => {
            if let Ok(string) = response.into_string() {
                let parse: Result<NoteResult, DeJsonErr> =
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
pub struct NoteResult {
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
