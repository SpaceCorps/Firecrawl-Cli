//! `firecrawl map` command.

use serde_json::{Map, Value};

use crate::account;
use crate::cli::MapArgs;
use crate::error::Result;
use crate::output;

pub fn run(args: MapArgs) -> Result<()> {
    let resolved = account::resolve(&args.auth)?;
    let client = resolved.client();

    let mut body = Map::new();
    body.insert("url".into(), Value::String(args.url));

    if let Some(search) = args.search {
        body.insert("search".into(), Value::String(search));
    }

    if let Some(sitemap) = args.sitemap {
        body.insert("sitemap".into(), Value::String(sitemap));
    }

    if let Some(inc_sub) = args.include_subdomains {
        body.insert("includeSubdomains".into(), Value::Bool(inc_sub));
    }

    if let Some(iqp) = args.ignore_query_params {
        body.insert("ignoreQueryParameters".into(), Value::Bool(iqp));
    }

    if args.ignore_cache {
        body.insert("ignoreCache".into(), Value::Bool(true));
    }

    if let Some(limit) = args.limit {
        body.insert("limit".into(), Value::from(limit));
    }

    if let Some(timeout) = args.timeout {
        body.insert("timeout".into(), Value::from(timeout));
    }

    let result = client.post("map", &Value::Object(body))?;
    output::write(&result);
    Ok(())
}
