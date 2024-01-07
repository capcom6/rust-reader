use anyhow::{bail, Result};

pub fn read_url(client: &reqwest::blocking::Client, url: &str) -> Result<Vec<u8>> {
    let response = client.get(url).send()?;
    if !response.status().is_success() {
        bail!("failed to read {}: {:?}", url, response.status());
    }

    Ok(response.bytes()?.to_vec())
}

pub fn strip_tags(text: &str) -> String {
    let re = regex::Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(text, "").to_string()
}
