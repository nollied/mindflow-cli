use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[derive(Clone)]
pub(crate) struct Reference {
    hash: String,
    text: String,
    r#type: String,
    path: String,
}

impl Reference {
    pub fn new(hash: String, text: String, r#type: String, path: String) -> Reference {
        Reference {
            hash,
            text,
            r#type,
            path,
        }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn get_type(&self) -> &String {
        &self.r#type
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }
}
