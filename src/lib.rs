use std::io::{BufRead, BufReader};

use anyhow::Result;
use clap::Parser;
use rss::Channel;
use time::OffsetDateTime;
use utils::{read_url, strip_tags};

mod utils;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.142.86 Safari/537.36";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of items to read
    #[arg(short, long)]
    count: Option<usize>,

    /// URLs to read
    #[arg(required = true)]
    urls: Vec<String>,
}

struct Item {
    title: String,
    link: String,
    description: Option<String>,
    pub_date: time::OffsetDateTime,
}

pub fn get_args() -> Args {
    Args::parse()
}

pub fn run(args: Args) -> Result<()> {
    let print = |(index, item): (usize, &Item)| {
        println!("{:>3}: ({}) {}", index + 1, item.pub_date, item.title);
        if let Some(desc) = &item.description {
            println!("\t{}", desc);
        }
        println!("\t{}", item.link);
        println!();
    };

    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .expect("failed to create client");
    let mut items = Vec::new();
    for url in args.urls {
        let next_items =
            read_url(&client, &url).and_then(|data| parse_rss(BufReader::new(&data[..])));
        match next_items {
            Ok(next_items) => {
                items.extend(next_items);
            }
            Err(err) => {
                eprintln!("failed to read {}: {}", url, err);
            }
        }
    }

    items.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

    items
        .iter()
        .take(args.count.unwrap_or(usize::MAX))
        .enumerate()
        .for_each(print);

    Ok(())
}

fn parse_rss<R: BufRead>(reader: R) -> Result<Vec<Item>> {
    let channel = Channel::read_from(reader)?;

    let items = channel
        .items()
        .iter()
        .map(|item| Item {
            title: item.title().unwrap().to_string(),
            link: item.link().unwrap().to_string(),
            description: item.description().map(strip_tags),
            pub_date: OffsetDateTime::parse(
                item.pub_date().unwrap(),
                &time::format_description::well_known::Rfc2822,
            )
            .expect("invalid date"),
        })
        .collect::<Vec<_>>();
    Ok(items)
}
