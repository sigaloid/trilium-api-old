#![deny(
    anonymous_parameters,
    clippy::all,
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_numeric_casts,
    unused_extern_crates
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::nursery,
    clippy::pedantic,
    clippy::todo,
    clippy::unimplemented,
    clippy::use_debug,
    clippy::all,
    unused_qualifications,
    variant_size_differences
)]
#![allow(dead_code)]
// Library has no tests as it would require a networked server.
use nanoserde::DeJsonErr;
pub mod schemas;
use nanoserde::SerJson;
use schemas::create_note_def::CreateNoteDef;
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
    fn new(password: &impl ToString, domain: &impl ToString) -> Result<Self, Error> {
        let req = ureq::post(&format!("{}/auth/login", domain.to_string()))
            .send_string(&format!("{{\"password\":\"{}\"}}", password.to_string()));
        match req {
            Ok(response) => {
                if let Ok(response_string) = response.into_string() {
                    let result: Result<LoginSchema, DeJsonErr> =
                        nanoserde::DeJson::deserialize_json(&response_string);
                    match result {
                        Ok(login_schema) => {
                            let middleware = move |req: Request, next: MiddlewareNext<'_>| {
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
    pub fn from_auth_key(auth: String, domain: &impl ToString) -> Result<Self, Error> {
        let middleware =
            move |req: Request, next: MiddlewareNext<'_>| next.handle(req.set("Authorization", &auth));
        let agent = ureq::builder().middleware(middleware).build();
        Ok(Self {
            agent,
            url: domain.to_string(),
        })
    }

    pub fn search_notes(
        &self,
        search_options: SearchOptions,
    ) -> Result<SearchResponse, Error> {
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

        let req = self
            .agent
            .get(&format!("{}/etapi/notes?{}", self.url, querystr))
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
                match e {
                    ureq::Error::Status(_, _) => Err(Error::WrongPassword),
                    ureq::Error::Transport(_) => Err(Error::InvalidUrl),
                }
            }
        }
    }

    pub fn create_note(&self, note: &CreateNoteDef) -> Result<CreateNoteResponse, Error> {
        let req = self
            .agent
            .post(&format!("{}/etapi/create-note", self.url))
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

    pub fn delete_note(&self, id: &impl ToString) -> Result<(), Error> {
        let req = self
            .agent
            .delete(&format!("{}/etapi/notes/{}", self.url, id.to_string()))
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

    pub fn get_note(&self, id: &impl ToString) -> Result<Note, Error> {
        let req = self
            .agent
            .get(&format!("{}/etapi/notes/{}", self.url, id.to_string()))
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
    pub fn patch_note(&self, note: &Note) -> Result<Note, Error> {
        let req = self
            .agent
            .patch(&format!("{}/etapi/notes/{}", self.url, note.note_id))
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
