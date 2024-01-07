use anyhow::{bail, Result};
use clap::Parser;
use rss::Channel;
use time::OffsetDateTime;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    urls: Vec<String>,
}

struct Item {
    title: String,
    link: String,
    description: Option<String>,
    pub_date: time::OffsetDateTime,
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    let printer = |(index, item): (usize, &Item)| {
        println!("{:>3}: ({}) {}", index, item.pub_date, item.title);
        if let Some(desc) = &item.description {
            println!("\t{}", desc);
        }
        println!("\t{}", item.link);
        println!();
    };

    let mut items = Vec::new();
    for url in args.urls {
        items.extend(read_url(&url)?);
    }

    items.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

    items.iter().enumerate().for_each(printer);

    Ok(())
}

fn read_url(url: &str) -> Result<Vec<Item>> {
    let response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        bail!("failed to read {}: {:?}", url, response.status());
    }

    let body = response.bytes()?;

    let channel = Channel::read_from(&body[..])?;

    let items = channel
        .items()
        .iter()
        .take(10)
        .map(|item| Item {
            title: item.title().unwrap().to_string(),
            link: item.link().unwrap().to_string(),
            description: item.description().map(|d| d.to_string()),
            pub_date: OffsetDateTime::parse(
                item.pub_date().unwrap(),
                &time::format_description::well_known::Rfc2822,
            )
            .expect("invalid date"),
        })
        .collect::<Vec<_>>();
    Ok(items)
}
