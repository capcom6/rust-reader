use std::fs;

use polodb_core::{bson::doc, Collection, Database};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Feed {
    pub title: String,
    pub url: String,
}

pub struct DB {
    db: Database,
    feeds: Collection<Feed>,
}

impl DB {
    pub fn new() -> Self {
        let home = dirs::home_dir().expect("failed to get home directory");
        let path = home.join(".rust-reader").join("db.polo");
        fs::create_dir_all(path.parent().unwrap()).expect("failed to create directory");
        let db = Database::open_file(&path).expect("failed to open database");
        let feeds = db.collection("feeds");

        Self { db, feeds }
    }

    pub fn add_feed(&mut self, feed: Feed) -> Result<String> {
        self.feeds
            .insert_one(feed)
            .map(|r| r.inserted_id.to_string())
            .map_err(|_| anyhow::anyhow!("failed to add feed"))
    }

    pub fn get_feeds(&self) -> Vec<Feed> {
        let result = self
            .feeds
            .find(None)
            .expect("failed to read feeds from database. This is a bug. Please report it.");
        result.map(|result| result.unwrap()).collect()
    }

    pub fn remove_feed(&mut self, url: &str) -> Result<()> {
        self.feeds
            .delete_one(doc! { "url": url })
            .map(|_| ())
            .map_err(|e| anyhow::anyhow!("failed to remove feed: {}", e))
    }

    // pub fn update_feed(&mut self, url: &str, title: String) {
    //     if let Some(feed) = self.get_feed_mut(url) {
    //         feed.title = title;
    //     }
    // }

    pub fn clear_feeds(&mut self) -> Result<()> {
        self.feeds
            .delete_many(doc! {})
            .map(|_| ())
            .map_err(|_| anyhow::anyhow!("failed to clear feeds"))
    }
}
