#![allow(dead_code)]
use create_note::Note;
use create_note::NoteResponse;
// Library has no tests as it would require a networked server.
use nanoserde::DeJsonErr;

mod create_note;
mod get_note;
mod patch_note;
pub mod search;

pub struct Trilium {
    url: String,
    agent: ureq::Agent,
}
impl Trilium {
    /// Create a new Trilium API agent with the given password.
    /// # Arguments
    ///
    /// * `password`: The password for the Trilium installation
    /// * `domain`: The domain for the Trilium installation (example: `https://example.com`)
    fn new(password: impl ToString, domain: impl ToString) -> Result<Trilium, Error> {
        let req = ureq::post(&format!("{}/auth/login", domain.to_string()))
            .send_string(&format!("{{\"password\":\"{}\"}}", password.to_string()));
        match req {
            Ok(response) => {
                if let Ok(response_string) = response.into_string() {
                    let result: Result<LoginSchema, DeJsonErr> =
                        nanoserde::DeJson::deserialize_json(&response_string);
                    match result {
                        Ok(login_schema) => {
                            let middleware = move |req: Request, next: MiddlewareNext| {
                                next.handle(req.set("Authorization", &login_schema.auth_token))
                            };
                            let agent = ureq::builder().middleware(middleware).build();
                            Ok(Self {
                                agent,
                                url: domain.to_string(),
                            })
                        }
                        Err(_) => Err(Error::InvalidServerResponse(Some(response_string))),
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
    pub fn from_auth_key(auth: String, domain: impl ToString) -> Result<Trilium, Error> {
        let middleware =
            move |req: Request, next: MiddlewareNext| next.handle(req.set("Authorization", &auth));
        let agent = ureq::builder().middleware(middleware).build();
        Ok(Self {
            agent,
            url: domain.to_string(),
        })
    }

    pub fn search_notes(
        trilium: &Trilium,
        search_options: SearchOptions,
    ) -> Result<SearchResults, crate::Error> {
        let mut querystr = String::new();
        querystr.push_str(&format!("search=\"{}\"", search_options.search));
        if search_options.fast_search {
            querystr.push_str("&fastSearch=true");
        }
        if search_options.include_archived_notes {
            querystr.push_str("&includeArchivedNotes=true");
        }
        if let Some(ancestor_id) = search_options.ancestor_note_id {
            querystr.push_str(&format!("&ancestorNoteId={}", ancestor_id));
        }
        if let Some(ancestor_depth) = search_options.ancestor_depth {
            querystr.push_str(&format!(
                "&ancestorDepth={}",
                match ancestor_depth {
                    search::DepthOptions::LessThan(a) => format!("lt{}", a),
                    search::DepthOptions::Exactly(a) => format!("eq{}", a),
                    search::DepthOptions::GreaterThan(a) => format!("gt{}", a),
                }
            ));
        }
        if let Some(order_by) = search_options.order_by {
            querystr.push_str(&format!("&orderBy={}", order_by));
        }
        if let Some(order_direction) = search_options.order_direction {
            querystr.push_str(&format!(
                "&orderDirection={}",
                match order_direction {
                    search::OrderDirection::Ascending => "asc",
                    search::OrderDirection::Descending => "dec",
                }
            ));
        }
        if let Some(limit) = search_options.limit {
            querystr.push_str(&format!("&limit={}", limit));
        }
        if search_options.debug {
            querystr.push_str("&debug=true");
        }

        let req = trilium
            .agent
            .get(&format!("{}/etapi/notes?{}", trilium.url, querystr))
            .call();
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
            Err(e) => {
                println!("{:?}", e);
                match e {
                    ureq::Error::Status(_, _) => Err(Error::WrongPassword),
                    ureq::Error::Transport(_) => Err(Error::InvalidUrl),
                }
            }
        }
    }

    pub fn create_note(trilium: &Self, note: Note) -> Result<NoteResponse, crate::Error> {
        let req = trilium
            .agent
            .post(&format!("{}/etapi/create-note", trilium.url))
            .send_string(&note.serialize_json());
        match req {
            Ok(response) => {
                if let Ok(string) = response.into_string() {
                    let parse: Result<NoteResponse, DeJsonErr> =
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

    pub fn delete_note(trilium: &Self, id: impl ToString) -> Result<(), crate::Error> {
        let req = trilium
            .agent
            .delete(&format!("{}/etapi/notes/{}", trilium.url, id.to_string()))
            .call();
        match req {
            Ok(response) => {
                if response.into_string().is_ok() {
                    Ok(())
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

    pub fn get_note(
        trilium: &Self,
        id: impl ToString,
    ) -> Result<get_note::NoteResult, crate::Error> {
        let req = trilium
            .agent
            .get(&format!("{}/etapi/notes/{}", trilium.url, id.to_string()))
            .call();
        match req {
            Ok(response) => {
                if let Ok(string) = response.into_string() {
                    let parse: Result<get_note::NoteResult, DeJsonErr> =
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
    pub fn patch_note(
        trilium: &Self,
        note: &patch_note::NoteResult,
    ) -> Result<patch_note::NoteResult, crate::Error> {
        let req = trilium
            .agent
            .patch(&format!("{}/etapi/notes/{}", trilium.url, note.note_id))
            .send_string(&note.serialize_json());
        match req {
            Ok(response) => {
                if let Ok(string) = response.into_string() {
                    let parse: Result<patch_note::NoteResult, DeJsonErr> =
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
}
#[derive(Debug)]
pub enum Error {
    InvalidUrl,
    WrongPassword,
    InvalidServerResponse(Option<String>),
}

use nanoserde::DeJson;
use nanoserde::SerJson;
use search::SearchOptions;
use search::SearchResults;
use ureq::MiddlewareNext;
use ureq::Request;

#[derive(DeJson)]
pub struct LoginSchema {
    #[nserde(rename = "authToken")]
    pub auth_token: String,
}

#[derive(SerJson, DeJson)]
pub struct ErrorResponse {
    #[nserde(rename = "status")]
    status: i64,

    #[nserde(rename = "code")]
    code: String,

    #[nserde(rename = "message")]
    message: String,
}
