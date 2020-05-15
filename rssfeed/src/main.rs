use bindings::prelude::*;

#[allow(unused)]
use windows::{foundation::*, web::syndication::*};

fn main() -> winrt::Result<()> {
    let feed_uri = Uri::create_uri("https://fasterthanli.me/index.xml")?;
    let client = SyndicationClient::new()?;
    let feed = client.retrieve_feed_async(feed_uri)?.blocking_get()?;

    let items = feed.items()?;

    let rust_items = items.into_iter().filter(|item| {
        let title = item.title().unwrap().text().unwrap().to_string();
        title.to_lowercase().contains("rust")
    });

    for item in rust_items.take(5) {
        println!("  - {}", item.title()?.text()?);
    }

    Ok(())
}
