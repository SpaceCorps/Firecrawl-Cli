//! The manual an agent reads before its first call. Markdown by default so it can be pasted
//! into a system prompt or a CLAUDE.md; `--json` gives the same rules as data.

use crate::{obj, output};

pub fn print() {
    if output::json() {
        output::write(&obj! {
            "tool" => "firecrawl",
            "apiVersion" => API_VERSION,
            "rules" => RULES,
            "exitCodes" => obj! {
                "0" => "ok",
                "1" => "error - unclassified, report and stop",
                "2" => "network - retry once, then stop",
                "3" => "auth_required - stop, surface the remediation to a human",
                "4" => "not_found - do not retry",
                "5" => "rate_limited - back off before retrying",
                "6" => "invalid_input - fix the call",
                "7" => "no_account - run firecrawl accounts list or login",
            },
        });
        return;
    }
    println!("{README}");
}

/// The Firecrawl v2 API this tool targets.
pub const API_VERSION: &str = "2.0.0";

const RULES: &[&str] = &[
    "Pass --account <name>, --api-key <key>, or set FIRECRAWL_API_KEY in your environment.",
    "Run 'firecrawl accounts list' to see configured accounts, or 'firecrawl login' to authenticate.",
    "On code auth_required, stop and surface the remediation string. Do not retry.",
    "Use --json when parsing command output programmatically or in agent tool loops.",
    "Crawl jobs are asynchronous: 'crawl start' returns an ID; use 'crawl status <ID>' to poll results.",
    "All errors print a structured envelope to stderr and exit with non-zero status.",
];

const README: &str = r#"# firecrawl - agent operating manual

A native CLI for the Firecrawl v2 API: web scraping, web crawling, sitemap mapping, and web search.
Results are YAML on stdout by default, errors are YAML on stderr, and `--json` switches both to JSON.
Prompts and warnings go to stderr, so stdout is always clean and safe to parse.

## Authentication & Accounts

Every command that accesses the Firecrawl API accepts either `--account <name>` (short `-a`)
using credentials stored securely in your OS keystore, or `--api-key <key>` (or `FIRECRAWL_API_KEY`).

### Managing accounts

    firecrawl login [<name>] [--api-key <key>]  # opens browser to copy API key
    firecrawl accounts add <name> --api-key <key> [--force]
    printf %s "$KEY" | firecrawl accounts add <name> --api-key-stdin
    firecrawl accounts list [--check]
    firecrawl accounts test <name>
    firecrawl accounts remove <name> --yes

`add` validates the key against the Firecrawl API before storing it in your operating system
keystore (macOS Keychain, Windows DPAPI, or Linux Secret Service). Non-secret metadata is stored
in `config.yaml`.

## Core Commands

### 1. Scrape — Extract content from a single URL

    firecrawl scrape <URL> [OPTIONS]

Options:
    --formats <FORMATS>        Comma-separated formats: markdown, html, rawHtml, screenshot, links
    --only-main-content        Exclude headers, navigation, and footers (default: true)
    --only-clean-content       LLM-based pass to remove residual boilerplate
    --include-tags <TAGS>      Comma-separated HTML tags to include
    --exclude-tags <TAGS>      Comma-separated HTML tags to exclude
    --wait-for <MS>            Milliseconds to wait before extracting content
    --timeout <MS>             Request timeout in milliseconds (default: 60000)
    --mobile                   Emulate a mobile device viewport
    --block-ads                Block ads and cookie popups
    --remove-base64-images     Strip base64 images from markdown output

Examples:
    firecrawl scrape https://example.com --formats markdown
    firecrawl scrape https://news.ycombinator.com --formats links --json

### 2. Crawl — Multi-page website crawling

    firecrawl crawl start <URL> [OPTIONS]   # start asynchronous crawl job
    firecrawl crawl status <ID>             # retrieve crawl progress and extracted pages
    firecrawl crawl cancel <ID>             # cancel an active crawl job

Start options:
    --limit <COUNT>            Maximum pages to crawl (default: 10000)
    --max-depth <DEPTH>        Maximum discovery depth
    --include-paths <REGEX>    Comma-separated path regex patterns to include
    --exclude-paths <REGEX>    Comma-separated path regex patterns to exclude
    --sitemap <MODE>           Sitemap mode: skip, include, only (default: include)
    --ignore-query-params      Prevent re-scraping same path with different query params
    --entire-domain            Follow sibling and parent URLs, not just subpaths
    --allow-external           Follow external website links
    --allow-subdomains         Follow subdomain links
    --delay <SECONDS>          Delay in seconds between page requests
    --formats <FORMATS>        Comma-separated output formats
    --only-main-content        Exclude headers, navigation, footers

Examples:
    firecrawl crawl start https://docs.rs/clap --limit 50 --max-depth 2
    firecrawl crawl status 550e8400-e29b-41d4-a716-446655440000 --json
    firecrawl crawl cancel 550e8400-e29b-41d4-a716-446655440000

### 3. Map — Discover URLs from a website

    firecrawl map <URL> [OPTIONS]

Options:
    --search <QUERY>           Query to filter and rank URLs by relevance
    --sitemap <MODE>           Sitemap mode: skip, include, only (default: include)
    --include-subdomains       Include subdomains in discovery
    --ignore-query-params      Exclude URLs containing query parameters
    --ignore-cache             Bypass the sitemap cache
    --limit <COUNT>            Maximum links to return (default: 5000, max: 100000)
    --timeout <MS>             Timeout in milliseconds

Examples:
    firecrawl map https://example.com
    firecrawl map https://example.com --search "blog" --limit 100

### 4. Search — Web search with page content extraction

    firecrawl search <QUERY> [OPTIONS]

Options:
    --limit <COUNT>            Number of results (1-100, default: 10)
    --country <CODE>           ISO country code (default: US)
    --location <LOCATION>      Geo-target location string
    --timeout <MS>             Request timeout in milliseconds (default: 60000)
    --formats <FORMATS>        Comma-separated scrape formats: markdown, html, rawHtml, links
    --only-main-content        Exclude headers, navigation, and footers
    --mobile                   Emulate a mobile device for scraping

Examples:
    firecrawl search "Rust 2024 edition features" --limit 5
    firecrawl search "AI web scrapers" --formats markdown --json

## Exit Codes & Errors

Failures print YAML (or JSON) to stderr with a machine-readable `code` and matching exit status:

    0  ok             Success
    1  error          Unclassified error - report and stop
    2  network        Network failure or timeout - retry once, then stop
    3  auth_required  Invalid API key or missing permissions - do not retry
    4  not_found      Target resource or job ID not found - do not retry
    5  rate_limited   Rate limit exceeded - back off before retrying
    6  invalid_input  Invalid arguments or parameters - fix the call
    7  no_account     No account or API key found - run firecrawl login or accounts list
"#;
