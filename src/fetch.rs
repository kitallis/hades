use rss::Channel;
use std::error::Error;

#[tokio::main]
pub async fn fetch_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;

    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
