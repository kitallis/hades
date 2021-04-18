use crate::config::{Author, Setting};
use crate::entry::Entry;
use crate::fetch::fetch_feed;
use rss::Channel;

#[derive(Debug)]
pub struct Feed {
    author: Author,
    setting: Setting,
    channel: Channel,
}

impl Feed {
    pub fn new(author: Author, setting: Setting) -> Self {
        println!("Fetching for {} from {}", author.name, author.feed);

        match fetch_feed(&author.feed) {
            Ok(channel) => Self { channel, author, setting },
            Err(_) => panic!("Failed to fetch for author: {}", author.name),
        }
    }

    pub fn write(&self) {
        self.parse()
            .iter()
            .for_each(|entry| {
                println!("Writing entry...");
                entry.write()
            })
    }

    fn parse(&self) -> Vec<Entry> {
        self.channel
            .items()
            .iter()
            .fold(Vec::new(), |mut entries, item| {
                let entry_metadata = Entry {
                    entry: item.clone(),
                    author: self.author.clone(),
                    setting: self.setting.clone(),
                };

                if entry_metadata.validate() {
                    entries.push(entry_metadata)
                } else {
                    println!("Skipping this entry...")
                }

                entries
            })
    }
}
