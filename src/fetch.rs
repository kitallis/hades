use rss::Channel;
use std::error::Error;
use url::{Url, ParseError};

#[tokio::main]
pub async fn fetch_feed(url: &Url) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url.as_str()).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
