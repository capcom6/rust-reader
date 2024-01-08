use anyhow::{bail, Result};
use terminal_menu::{back_button, button, label, menu, mut_menu, run, scroll, string, submenu};

use crate::db::{Feed, DB};

pub struct Menu {
    db: DB,
}

impl Menu {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub fn run(&mut self) -> Result<()> {
        let menu = menu(vec![
            label("Welcome to Rust RSS Feed Reader"),
            label(""),
            label("What do you want to do?"),
            label(""),
            //
            button("Read Latest News"),
            button("Read Single Feed"),
            button("Edit Feeds"),
            back_button("Quit (q)"),
        ]);

        loop {
            run(&menu);

            let mut_menu = mut_menu(&menu);
            let selected_index = mut_menu.selected_item_index();

            match selected_index {
                4 => {
                    // menu::read_latest();
                    println!("Read Latest News");
                }
                5 => {
                    // menu::read_single();
                    println!("Read Single Feed");
                }
                6 => self.edit_feeds(),
                7 => {
                    return Ok(());
                }
                _ => {
                    bail!("Invalid index: {}", selected_index);
                }
            }
        }
    }

    fn edit_feeds(&mut self) {
        loop {
            let feeds = self.db.get_feeds();
            let mut menu_items = vec![
                label("Edit Feeds"),
                label(""),
                //
                button("Add Feed..."),
            ];

            let first_feed_index = menu_items.len();
            for (index, feed) in feeds.iter().enumerate() {
                menu_items.push(button(format!("{}: {}", index + 1, feed.title)));
            }

            menu_items.push(back_button("Back"));

            let menu = menu(menu_items);

            run(&menu);

            let mut_menu = mut_menu(&menu);
            let selected = mut_menu.selected_item_name();
            println!("Selected: {}", selected);

            if selected == "Back" {
                break;
            }

            if selected == "Add Feed..." {
                self.add_feed();
                continue;
            }

            let index = mut_menu.selected_item_index() - first_feed_index;
            let feed = &feeds[index];
            self.edit_feed(feed);
        }
    }

    fn add_feed(&mut self) {
        let menu = menu(vec![
            label("Add Feed"),
            label(""),
            //
            string("Title", "", false),
            string("URL", "", false),
            label(""),
            //
            button("Add"),
            back_button("Cancel"),
        ]);

        loop {
            run(&menu);

            let mut_menu = mut_menu(&menu);
            let selected = mut_menu.selected_item_name();

            if selected == "Cancel" {
                break;
            }

            let title = mut_menu.selection_value("Title");
            let url = mut_menu.selection_value("URL");

            let feed = Feed {
                title: title.into(),
                url: url.into(),
            };

            self.db.add_feed(feed);
            break;
        }
    }

    fn edit_feed(&mut self, feed: &Feed) {
        let menu = menu(vec![
            label("Edit Feed"),
            label(""),
            //
            label("Title: ".to_owned() + feed.title.as_str()),
            label("URL: ".to_owned() + feed.url.as_str()),
            label(""),
            //
            scroll("Remove", vec!["No", "Yes"]),
            back_button("Done"),
        ]);

        run(&menu);

        let mut_menu = mut_menu(&menu);
        let selected = mut_menu.selected_item_name();

        if mut_menu.selection_value("Remove") == "Yes" {
            self.db.remove_feed(feed.url.as_str());
        }
    }
}
