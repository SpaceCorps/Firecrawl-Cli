//! `firecrawl crawl` commands: `start`, `status`, `cancel`.

use serde_json::{Map, Value};

use crate::account;
use crate::cli::{CrawlCancelArgs, CrawlCommand, CrawlStartArgs, CrawlStatusArgs};
use crate::commands::scrape::split_csv;
use crate::error::Result;
use crate::output;

pub fn run(cmd: CrawlCommand) -> Result<()> {
    match cmd {
        CrawlCommand::Start(args) => start(args),
        CrawlCommand::Status(args) => status(args),
        CrawlCommand::Cancel(args) => cancel(args),
    }
}

fn start(args: CrawlStartArgs) -> Result<()> {
    let resolved = account::resolve(&args.auth)?;
    let client = resolved.client();

    let mut body = Map::new();
    body.insert("url".into(), Value::String(args.url));

    if let Some(limit) = args.limit {
        body.insert("limit".into(), Value::from(limit));
    }

    if let Some(depth) = args.max_depth {
        body.insert("maxDiscoveryDepth".into(), Value::from(depth));
    }

    if let Some(sitemap) = args.sitemap {
        body.insert("sitemap".into(), Value::String(sitemap));
    }

    if args.ignore_query_params {
        body.insert("ignoreQueryParameters".into(), Value::Bool(true));
    }

    if args.entire_domain {
        body.insert("crawlEntireDomain".into(), Value::Bool(true));
    }

    if args.allow_external {
        body.insert("allowExternalLinks".into(), Value::Bool(true));
    }

    if args.allow_subdomains {
        body.insert("allowSubdomains".into(), Value::Bool(true));
    }

    if let Some(delay) = args.delay
        && let Some(v) = serde_json::Number::from_f64(delay)
    {
        body.insert("delay".into(), Value::Number(v));
    }

    if let Some(include_paths) = args.include_paths {
        let list: Vec<Value> = split_csv(&include_paths).into_iter().map(Value::String).collect();
        body.insert("includePaths".into(), Value::Array(list));
    }

    if let Some(exclude_paths) = args.exclude_paths {
        let list: Vec<Value> = split_csv(&exclude_paths).into_iter().map(Value::String).collect();
        body.insert("excludePaths".into(), Value::Array(list));
    }

    let mut scrape_options = Map::new();
    if let Some(formats) = args.formats {
        let list: Vec<Value> = split_csv(&formats).into_iter().map(Value::String).collect();
        scrape_options.insert("formats".into(), Value::Array(list));
    }

    if let Some(omc) = args.only_main_content {
        scrape_options.insert("onlyMainContent".into(), Value::Bool(omc));
    }

    if !scrape_options.is_empty() {
        body.insert("scrapeOptions".into(), Value::Object(scrape_options));
    }

    let result = client.post("crawl", &Value::Object(body))?;
    output::write(&result);
    Ok(())
}

fn status(args: CrawlStatusArgs) -> Result<()> {
    let resolved = account::resolve(&args.auth)?;
    let client = resolved.client();

    let path = format!("crawl/{}", args.id.trim());
    let result = client.get(&path)?;
    output::write(&result);
    Ok(())
}

fn cancel(args: CrawlCancelArgs) -> Result<()> {
    let resolved = account::resolve(&args.auth)?;
    let client = resolved.client();

    let path = format!("crawl/{}", args.id.trim());
    let result = client.delete(&path)?;
    output::write(&result);
    Ok(())
}
