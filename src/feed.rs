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
    pub fn new(author: Author, setting: Setting) -> Option<Self> {
        if author.enabled {
            match fetch_feed(&author.feed) {
                Ok(channel) => Some(Self { channel, author, setting }),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn write(&self) {
        self.parse()
            .iter()
            .for_each(|entry| entry.write())
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
