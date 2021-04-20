use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Preamble {
    pub title: String,
    pub author: String,
    pub created_at: String,
}

impl Preamble {
    pub fn toml(&self) -> String {
        match toml::to_string(self) {
            Ok(string) => format!("+++\n{}+++", string),
            Err(_) => String::new(),
        }
    }

    pub fn yaml(&self) -> String {
        match serde_yaml::to_string(self) {
            Ok(string) => format!("{}---", string),
            Err(_) => String::new(),
        }
    }
}
