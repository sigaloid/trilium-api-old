use crate::{Error, Trilium};

pub fn delete_note(trilium: &Trilium, id: impl ToString) -> Result<(), crate::Error> {
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
