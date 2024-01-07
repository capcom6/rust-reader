use std::io::{BufRead, BufReader};

use anyhow::Result;
use clap::Parser;
use rss::Channel;
use time::OffsetDateTime;
use utils::strip_tags;

mod utils;

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
    // let args = Args::parse();

    let print = |(index, item): (usize, &Item)| {
        println!("{:>3}: ({}) {}", index + 1, item.pub_date, item.title);
        if let Some(desc) = &item.description {
            println!("\t{}", desc);
        }
        println!("\t{}", item.link);
        println!();
    };

    let mut items = Vec::new();
    for url in args.urls {
        let data = utils::read_url(&url).expect("failed to read URL");
        items.extend(parse_rss(BufReader::new(&data[..]))?);
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
            description: item.description().map(|d| strip_tags(d)),
            pub_date: OffsetDateTime::parse(
                item.pub_date().unwrap(),
                &time::format_description::well_known::Rfc2822,
            )
            .expect("invalid date"),
        })
        .collect::<Vec<_>>();
    Ok(items)
}
