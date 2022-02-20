use nanoserde::DeJson;

#[derive(DeJson)]
pub struct LoginSchema {
    #[nserde(rename = "authToken")]
    pub auth_token: String,
}