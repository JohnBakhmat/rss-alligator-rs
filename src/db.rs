use crate::feed_parser::{self, Post};
use crate::prisma::{feed, post, subscription, user, PrismaClient};
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

pub async fn find_or_upsert_feed(feed: &feed_parser::Feed) -> anyhow::Result<feed::Data> {
    let client = PrismaClient::_builder().build().await?;

    let feed_exists = client
        .feed()
        .find_first(vec![feed::link::equals(feed.link.to_string())])
        .exec()
        .await?;

    if feed_exists.is_some() {
        let feed_id = &feed_exists.as_ref().unwrap().id;
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
            return Ok(feed_exists.unwrap());
        }

        for post in new_posts {
            add_post(feed_id.to_string(), post).await?;
        }
    } else {
        let new_feed = client
            .feed()
            .create(feed.title.to_string(), feed.link.to_string(), vec![])
            .exec()
            .await?;

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
                            vec![post::feed_id::set(Some(new_feed.id.to_string()))],
                        )
                    })
                    .collect(),
            )
            .exec()
            .await?;
    }

    let updated_feed = client
        .feed()
        .find_first(vec![feed::id::equals(feed_exists.unwrap().id.to_string())])
        .exec()
        .await?;

    return Ok(updated_feed.unwrap());
}

pub async fn subscribe_user_to_feed(
    username: &str,
    feed: &feed_parser::Feed,
) -> anyhow::Result<subscription::Data> {
    let user = find_or_create_user(username).await?;
    let feed = find_or_upsert_feed(feed).await?;

    let client = PrismaClient::_builder().build().await?;

    let user_feed = client
        .subscription()
        .find_first(vec![
            subscription::user_id::equals(user.id.to_string()),
            subscription::feed_id::equals(feed.id.to_string()),
        ])
        .exec()
        .await?;

    if user_feed.is_some() {
        return Ok(user_feed.unwrap());
    }

    let new_sub = client
        .subscription()
        .create(
            user::id::equals(user.id.to_string()),
            feed::id::equals(feed.id.to_string()),
            vec![],
        )
        .exec()
        .await?;

    Ok(new_sub)
}

pub async fn find_or_create_user(username: &str) -> anyhow::Result<user::Data> {
    let client = PrismaClient::_builder().build().await?;

    let user_exists = client
        .user()
        .find_first(vec![user::username::equals(username.to_string())])
        .exec()
        .await?;

    if user_exists.is_some() {
        return Ok(user_exists.unwrap());
    }

    let user = client
        .user()
        .create(username.to_string(), vec![])
        .exec()
        .await?;

    Ok(user)
}
