pub struct Feed {
    pub title: String,
    pub url: String,
}

pub struct DB {
    feeds: Vec<Feed>,
}

impl DB {
    pub fn new() -> Self {
        Self { feeds: Vec::new() }
    }

    pub fn add_feed(&mut self, feed: Feed) {
        self.feeds.push(feed);
    }

    pub fn get_feeds(&self) -> &Vec<Feed> {
        &self.feeds
    }

    pub fn remove_feed(&mut self, url: &str) {
        self.feeds.retain(|feed| feed.url != url);
    }

    // pub fn update_feed(&mut self, url: &str, title: String) {
    //     if let Some(feed) = self.get_feed_mut(url) {
    //         feed.title = title;
    //     }
    // }

    pub fn clear_feeds(&mut self) {
        self.feeds.clear();
    }
}
