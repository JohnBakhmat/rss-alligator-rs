mod feed_parser;

#[allow(warnings, unused)]
mod prisma;

use std::vec;

use prisma::{feed, post, PrismaClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let nexxel = feed_parser::get_feed_by_url("https://nexxel.dev/rss.xml").await?;
    save_feed_to_db(&nexxel).await?;
    Ok(())
}

async fn save_feed_to_db(feed: &feed_parser::Feed) -> anyhow::Result<()> {
    let client = PrismaClient::_builder().build().await?;

    client
        .feed()
        .create(feed.title.to_string(), feed.link.to_string(), vec![])
        .exec()
        .await?;

    let feed_id = client
        .feed()
        .find_first(vec![feed::link::equals(feed.link.to_string())])
        .exec()
        .await?
        .unwrap()
        .id;

    client
        .post()
        .create_many(
            feed.items
                .iter()
                .map(|x| {
                    post::create_unchecked(
                        x.title.to_string(),
                        x.link.to_string(),
                        x.pub_date.to_string(),
                        vec![post::feed_id::set(Some(feed_id.to_string()))],
                    )
                })
                .collect(),
        )
        .exec()
        .await?;

    Ok(())
}
