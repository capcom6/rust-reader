use anyhow::{bail, Result};

pub fn read_url(url: &str) -> Result<Vec<u8>> {
    let response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        bail!("failed to read {}: {:?}", url, response.status());
    }

    Ok(response.bytes()?.to_vec())
}

pub fn strip_tags(text: &str) -> String {
    let re = regex::Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(text, "").to_string()
}
