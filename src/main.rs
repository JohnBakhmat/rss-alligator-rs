mod db;
mod feed_parser;

#[allow(warnings, unused)]
mod prisma;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let nexxel = feed_parser::get_feed_by_url("https://nexxel.dev/rss.xml").await?;
    db::save_feed_to_db(&nexxel).await?;
    Ok(())
}
