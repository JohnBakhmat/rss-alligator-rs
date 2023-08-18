mod feed;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let nexxel = feed::get_feed_by_url("https://nexxel.dev/rss.xml").await?;
    println!("{}", nexxel.to_string());
    Ok(())
}
