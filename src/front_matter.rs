use chrono::{DateTime, FixedOffset};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub author: String,
    pub date: DateTime<FixedOffset>,
    pub link: String,
}

#[derive(Debug, PartialEq, Serialize)]
struct Zola {
    title: String,
    date: DateTime<FixedOffset>,
    extra: ZolaExtra,
}

#[derive(Debug, PartialEq, Serialize)]
struct ZolaExtra {
    author: String,
    link: String,
}

impl Zola {
    pub fn toml(&self) -> String {
        match toml::to_string(self) {
            Ok(string) => format!("+++\n{}+++", string),
            Err(_) => String::new(),
        }
    }
}

impl FrontMatter {
    pub fn zola_toml(&self) -> String {
        let zola = Zola {
            title: self.title.to_string(),
            date: self.date.clone(),
            extra: ZolaExtra {
                author: self.author.to_string(),
                link: self.link.to_string(),
            },
        };

        zola.toml()
    }

    pub fn yaml(&self) -> String {
        match serde_yaml::to_string(self) {
            Ok(string) => format!("{}---", string),
            Err(_) => String::new(),
        }
    }
}
