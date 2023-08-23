mod db;
mod feed_parser;

#[allow(warnings, unused)]
mod prisma;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    handle_sub("test", "https://nexxel.dev/rss.xml").await?;
    Ok(())
}

async fn handle_sub(username: &str, url: &str) -> anyhow::Result<()> {
    let feed = feed_parser::get_feed_by_url(url).await?;
    db::subscribe_user_to_feed(username, &feed).await?;
    Ok(())
}
