#![allow(dead_code)]
use anyhow;
use reqwest;
use rocksdb::DB;
use std::time::Duration;

pub fn fetch_url(url: &str, db: &DB, retry_failed: bool) -> anyhow::Result<String> {
    let key = format!("fetch:{}", url);
    let value = db.get(key.as_bytes()).unwrap();

    if let Some(value) = value {
        let content = String::from_utf8(value.clone()).unwrap();
        if content == "failed" {
            if !retry_failed {
                return Err(anyhow::anyhow!("Failed to fetch url"));
            }
        } else {
            return Ok(String::from_utf8(value).unwrap());
        }
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Mobile Safari/537.36")
        .timeout(Duration::from_secs(5))
        .build()?;

    let req = client.get(url).send();
    if req.is_err() {
        db.put(key.as_bytes(), "failed".as_bytes()).unwrap();
        return Err(anyhow::anyhow!("Failed to fetch url"));
    }
    let content = req?.text()?;
    db.put(key.as_bytes(), content.as_bytes()).unwrap();
    Ok(content)
}
