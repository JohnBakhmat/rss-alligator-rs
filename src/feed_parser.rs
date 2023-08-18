use rss::Channel;

pub struct Feed {
    pub title: String,
    pub link: String,
    pub items: Vec<Post>,
}
pub struct Post {
    pub title: String,
    pub link: String,
    pub pub_date: String,
}

#[allow(unused)]
impl Post {
    pub fn to_string(&self) -> String {
        format!("*\t{}\t{}\t{}\n", self.title, self.link, self.pub_date)
    }
}

#[allow(unused)]
impl Feed {
    pub fn to_string(&self) -> String {
        let mut result = format!("{}\t{}\n", self.title, self.link);
        for item in &self.items {
            result.push_str(&item.to_string());
        }
        result
    }
}

pub async fn get_feed_by_url(url: &str) -> anyhow::Result<Feed> {
    let channel = get_channel_by_url(url).await?;
    let feed = channel_to_feed(&channel).await?;

    Ok(feed)
}

async fn get_channel_by_url(url: &str) -> anyhow::Result<Channel> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

async fn channel_to_feed(channel: &Channel) -> anyhow::Result<Feed> {
    let items = channel
        .items()
        .iter()
        .map(|item| Post {
            title: item.title().unwrap_or_default().to_string(),
            link: item.link().unwrap_or_default().to_string(),
            pub_date: item.pub_date().unwrap_or_default().to_string(),
        })
        .collect();

    let feed = Feed {
        title: channel.title().to_string(),
        link: channel.link().to_string(),
        items,
    };

    Ok(feed)
}
