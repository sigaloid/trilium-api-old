#![allow(dead_code)]
// Library has no tests as it would require a networked server.
use nanoserde::DeJsonErr;
mod schemas;
use nanoserde::SerJson;
use schemas::create_note_response::CreateNoteResponse;
use schemas::login::LoginSchema;
use schemas::note::Note;
use schemas::search_options::DepthOptions;
use schemas::search_options::OrderDirection;
use schemas::search_options::SearchOptions;
use schemas::search_response::SearchResponse;
use ureq::MiddlewareNext;
use ureq::Request;

pub struct Trilium {
    url: String,
    agent: ureq::Agent,
}
impl Trilium {
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
    ) -> Result<SearchResponse, crate::Error> {
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
                    DepthOptions::LessThan(a) => format!("lt{}", a),
                    DepthOptions::Exactly(a) => format!("eq{}", a),
                    DepthOptions::GreaterThan(a) => format!("gt{}", a),
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
                    OrderDirection::Ascending => "asc",
                    OrderDirection::Descending => "dec",
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
                    let parse: Result<SearchResponse, DeJsonErr> =
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

    pub fn create_note(trilium: &Self, note: CreateNote) -> Result<CreateNoteResponse, crate::Error> {
        let req = trilium
            .agent
            .post(&format!("{}/etapi/create-note", trilium.url))
            .send_string(&note.serialize_json());
        match req {
            Ok(response) => {
                if let Ok(string) = response.into_string() {
                    let parse: Result<CreateNoteResponse, DeJsonErr> =
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
    ) -> Result<Note, crate::Error> {
        let req = trilium
            .agent
            .get(&format!("{}/etapi/notes/{}", trilium.url, id.to_string()))
            .call();
        match req {
            Ok(response) => {
                if let Ok(string) = response.into_string() {
                    let parse: Result<Note, DeJsonErr> =
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
        note: &Note,
    ) -> Result<Note, crate::Error> {
        let req = trilium
            .agent
            .patch(&format!("{}/etapi/notes/{}", trilium.url, note.note_id.id))
            .send_string(&note.serialize_json());
        match req {
            Ok(response) => {
                if let Ok(string) = response.into_string() {
                    let parse: Result<Note, DeJsonErr> =
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


