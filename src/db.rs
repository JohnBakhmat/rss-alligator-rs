use crate::feed_parser::{self, Post};
use crate::prisma::{feed, post, PrismaClient};
use std::vec;

pub async fn add_post(feed_id: String, post: &Post) -> anyhow::Result<()> {
    let client = PrismaClient::_builder().build().await?;

    let feed = client
        .feed()
        .find_first(vec![feed::id::equals(feed_id.to_string())])
        .exec()
        .await?;

    if let Some(feed) = feed {
        client
            .post()
            .create(
                post.title.to_string(),
                post.link.to_string(),
                post.pub_date.to_string(),
                vec![post::feed_id::set(Some(feed.id.to_string()))],
            )
            .exec()
            .await?;
    }

    Ok(())
}

pub async fn save_feed_to_db(feed: &feed_parser::Feed) -> anyhow::Result<()> {
    let client = PrismaClient::_builder().build().await?;

    let feed_exists = client
        .feed()
        .find_first(vec![feed::link::equals(feed.link.to_string())])
        .exec()
        .await?;

    if feed_exists.is_some() {
        let feed_id = feed_exists.unwrap().id;
        let posts = client
            .post()
            .find_many(vec![post::feed_id::equals(Some(feed_id.to_string()))])
            .exec()
            .await?;

        let mut new_posts = vec![];

        for item in &feed.items {
            let mut exists = false;
            for post in &posts {
                if post.link == item.link {
                    exists = true;
                    break;
                }
            }
            if !exists {
                new_posts.push(item);
            }
        }

        if new_posts.len() == 0 {
            return Ok(());
        }

        for post in new_posts {
            add_post(feed_id.to_string(), post).await?;
        }
        return Ok(());
    }

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
