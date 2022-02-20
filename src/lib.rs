#![allow(dead_code)] // Library has no tests as it would require a networked server.
use nanoserde::DeJsonErr;

mod create_note;
mod delete_note;
mod get_note;
mod patch_note;
mod search;

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
    fn from_auth_key(auth: String, domain: impl ToString) -> Result<Trilium, Error> {
        let middleware =
            move |req: Request, next: MiddlewareNext| next.handle(req.set("Authorization", &auth));
        let agent = ureq::builder().middleware(middleware).build();
        Ok(Self {
            agent,
            url: domain.to_string(),
        })
    }
}
pub enum Error {
    InvalidUrl,
    WrongPassword,
    InvalidServerResponse(Option<String>),
}

use nanoserde::DeJson;
use nanoserde::SerJson;
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
