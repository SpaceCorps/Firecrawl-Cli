//! `firecrawl search` command.

use serde_json::{Map, Value};

use crate::account;
use crate::cli::SearchArgs;
use crate::commands::scrape::split_csv;
use crate::error::Result;
use crate::output;

pub fn run(args: SearchArgs) -> Result<()> {
    let resolved = account::resolve(&args.auth)?;
    let client = resolved.client();

    let mut body = Map::new();
    body.insert("query".into(), Value::String(args.query));

    if let Some(limit) = args.limit {
        body.insert("limit".into(), Value::from(limit));
    }

    if let Some(country) = args.country {
        body.insert("country".into(), Value::String(country));
    }

    if let Some(location) = args.location {
        body.insert("location".into(), Value::String(location));
    }

    if let Some(timeout) = args.timeout {
        body.insert("timeout".into(), Value::from(timeout));
    }

    let mut scrape_options = Map::new();
    if let Some(formats) = args.formats {
        let list: Vec<Value> = split_csv(&formats).into_iter().map(Value::String).collect();
        scrape_options.insert("formats".into(), Value::Array(list));
    }

    if args.only_main_content {
        scrape_options.insert("onlyMainContent".into(), Value::Bool(true));
    }

    if args.mobile {
        scrape_options.insert("mobile".into(), Value::Bool(true));
    }

    if !scrape_options.is_empty() {
        body.insert("scrapeOptions".into(), Value::Object(scrape_options));
    }

    let result = client.post("search", &Value::Object(body))?;
    output::write(&result);
    Ok(())
}
