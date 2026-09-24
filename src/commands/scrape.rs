//! `firecrawl scrape` command.

use serde_json::{Map, Value};

use crate::account;
use crate::cli::ScrapeArgs;
use crate::error::Result;
use crate::output;

pub fn run(args: ScrapeArgs) -> Result<()> {
    let resolved = account::resolve(&args.auth)?;
    let client = resolved.client();

    let mut body = Map::new();
    body.insert("url".into(), Value::String(args.url));

    if let Some(formats) = args.formats {
        let list: Vec<Value> = split_csv(&formats).into_iter().map(Value::String).collect();
        body.insert("formats".into(), Value::Array(list));
    }

    if let Some(omc) = args.only_main_content {
        body.insert("onlyMainContent".into(), Value::Bool(omc));
    }

    if args.only_clean_content {
        body.insert("onlyCleanContent".into(), Value::Bool(true));
    }

    if let Some(include_tags) = args.include_tags {
        let list: Vec<Value> = split_csv(&include_tags).into_iter().map(Value::String).collect();
        body.insert("includeTags".into(), Value::Array(list));
    }

    if let Some(exclude_tags) = args.exclude_tags {
        let list: Vec<Value> = split_csv(&exclude_tags).into_iter().map(Value::String).collect();
        body.insert("excludeTags".into(), Value::Array(list));
    }

    if let Some(wait_for) = args.wait_for {
        body.insert("waitFor".into(), Value::from(wait_for));
    }

    if let Some(timeout) = args.timeout {
        body.insert("timeout".into(), Value::from(timeout));
    }

    if args.mobile {
        body.insert("mobile".into(), Value::Bool(true));
    }

    if let Some(block_ads) = args.block_ads {
        body.insert("blockAds".into(), Value::Bool(block_ads));
    }

    if let Some(remove_b64) = args.remove_base64_images {
        body.insert("removeBase64Images".into(), Value::Bool(remove_b64));
    }

    let result = client.post("scrape", &Value::Object(body))?;
    output::write(&result);
    Ok(())
}

pub fn split_csv(s: &str) -> Vec<String> {
    s.split(',').map(str::trim).filter(|part| !part.is_empty()).map(str::to_string).collect()
}
