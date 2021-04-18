use rss::Item;
use chrono::DateTime;
use std::path::{Path, PathBuf};
use crate::config::{Author, Setting};
use slug::slugify;
use std::fs::File;
use std::io::prelude::*;

const ENTRY_EXT: &str = "md";

#[derive(Debug, Clone)]
pub struct Entry {
    pub entry: Item,
    pub author: Author,
    pub setting: Setting,
}

impl Entry {
    pub fn preamble(&self) -> String {
        format!("\
        ---\
        \n\
        title: {}\
        \n\
        author: {}\
        \n\
        created_at: {}\
        \n\
        ---", self.entry.title().unwrap(), self.default_author(), self.entry.pub_date().unwrap())
    }

    pub fn body(&self) -> String {
        format!("{}\n{}", self.preamble(), self.default_content())
    }

    pub fn name(&self) -> PathBuf {
        let time = DateTime::parse_from_rfc2822(self.entry.pub_date().unwrap()).unwrap().format("%Y-%m-%d");
        let directory = "out";
        let title = slugify(self.entry.title().unwrap());
        let file_name = format!("{}-{}", time, title);

        Path::new(directory)
            .join(&file_name)
            .with_extension(ENTRY_EXT)
    }

    pub fn validate(&self) -> bool {
        true
    }

    pub fn default_author(&self) -> String {
        match self.entry.author() {
            Some(author) => author.to_owned(),
            None => self.author.name.to_owned()
        }
    }

    pub fn default_content(&self) -> String {
        match (self.entry.content(), self.entry.description()) {
            (Some(content), None) => content.to_owned(),
            (None, Some(description)) => description.to_owned(),
            _ => String::new()
        }
    }

    pub fn write(&self) {
        let file_path = &self.name();
        let display = file_path.display();

        let mut file = match File::create(&file_path) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't create {}: {}", display, why),
        };

        match file.write_all(self.body().as_bytes()) {
            Ok(_) => println!("Successfully wrote to: {}", display),
            Err(why) => panic!("Couldn't write to {}: {}", display, why),
        }
    }
}
