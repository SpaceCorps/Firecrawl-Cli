//! Command-line argument definitions and clap hierarchy.

use clap::{Args, Parser, Subcommand};

use crate::account::AuthArgs;

#[derive(Parser)]
#[command(
    name = "firecrawl",
    version,
    about = "CLI for the Firecrawl v2 API — web scraping, crawling, mapping, and web search for LLM agents",
    after_help = "An LLM agent should start with: firecrawl agent-readme",
    propagate_version = true,
    disable_help_subcommand = true
)]
pub struct Cli {
    /// Print raw JSON instead of YAML, for scripting
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Scrape a single URL and extract content
    Scrape(ScrapeArgs),

    /// Crawl websites across multiple pages
    #[command(subcommand)]
    Crawl(CrawlCommand),

    /// Generate a list of URLs from a website
    Map(MapArgs),

    /// Search the web and retrieve page content
    Search(SearchArgs),

    /// Log in with a Firecrawl API key (opens dashboard to copy key)
    Login(LoginArgs),

    /// Manage Firecrawl accounts and their API keys in the OS keystore
    #[command(subcommand)]
    Accounts(AccountsCommand),

    /// Print the operating manual for an LLM agent driving this CLI
    AgentReadme,
}

// ---------------------------------------------------------------------------------------------
// scrape

#[derive(Args, Clone, Debug)]
pub struct ScrapeArgs {
    #[command(flatten)]
    pub auth: AuthArgs,

    /// The URL to scrape
    #[arg(value_name = "URL")]
    pub url: String,

    /// Comma-separated output formats: markdown, html, rawHtml, screenshot, links
    #[arg(long, value_name = "FORMATS")]
    pub formats: Option<String>,

    /// Exclude headers, navigation, footers (default: true)
    #[arg(long, num_args = 0..=1, default_missing_value = "true")]
    pub only_main_content: Option<bool>,

    /// LLM-based pass to remove residual boilerplate
    #[arg(long)]
    pub only_clean_content: bool,

    /// Comma-separated HTML tags to include
    #[arg(long, value_name = "TAGS")]
    pub include_tags: Option<String>,

    /// Comma-separated HTML tags to exclude
    #[arg(long, value_name = "TAGS")]
    pub exclude_tags: Option<String>,

    /// Milliseconds to wait before extracting content
    #[arg(long, value_name = "MS")]
    pub wait_for: Option<u64>,

    /// Request timeout in milliseconds (default: 60000)
    #[arg(long, value_name = "MS")]
    pub timeout: Option<u64>,

    /// Emulate a mobile device
    #[arg(long)]
    pub mobile: bool,

    /// Block ads and cookie popups (default: true)
    #[arg(long, num_args = 0..=1, default_missing_value = "true")]
    pub block_ads: Option<bool>,

    /// Strip base64 images from markdown (default: true)
    #[arg(long, num_args = 0..=1, default_missing_value = "true")]
    pub remove_base64_images: Option<bool>,
}

// ---------------------------------------------------------------------------------------------
// crawl

#[derive(Subcommand, Clone, Debug)]
pub enum CrawlCommand {
    /// Start a new crawl job
    Start(CrawlStartArgs),

    /// Get crawl job status and results
    Status(CrawlStatusArgs),

    /// Cancel a running crawl job
    Cancel(CrawlCancelArgs),
}

#[derive(Args, Clone, Debug)]
pub struct CrawlStartArgs {
    #[command(flatten)]
    pub auth: AuthArgs,

    /// The base URL to start crawling from
    #[arg(value_name = "URL")]
    pub url: String,

    /// Maximum pages to crawl (default: 10000)
    #[arg(long, value_name = "COUNT")]
    pub limit: Option<u32>,

    /// Maximum discovery depth
    #[arg(long, value_name = "DEPTH")]
    pub max_depth: Option<u32>,

    /// Comma-separated URL path regex patterns to include
    #[arg(long, value_name = "PATTERNS")]
    pub include_paths: Option<String>,

    /// Comma-separated URL path regex patterns to exclude
    #[arg(long, value_name = "PATTERNS")]
    pub exclude_paths: Option<String>,

    /// Sitemap mode: skip, include, only (default: include)
    #[arg(long, value_name = "MODE")]
    pub sitemap: Option<String>,

    /// Prevent re-scraping same path with different query params
    #[arg(long)]
    pub ignore_query_params: bool,

    /// Follow sibling/parent URLs, not just child paths
    #[arg(long)]
    pub entire_domain: bool,

    /// Follow external website links
    #[arg(long)]
    pub allow_external: bool,

    /// Follow subdomain links
    #[arg(long)]
    pub allow_subdomains: bool,

    /// Delay in seconds between scrapes
    #[arg(long, value_name = "SECONDS")]
    pub delay: Option<f64>,

    /// Comma-separated output formats: markdown, html, rawHtml, links
    #[arg(long, value_name = "FORMATS")]
    pub formats: Option<String>,

    /// Exclude headers, navigation, footers (default: true)
    #[arg(long, num_args = 0..=1, default_missing_value = "true")]
    pub only_main_content: Option<bool>,
}

#[derive(Args, Clone, Debug)]
pub struct CrawlStatusArgs {
    #[command(flatten)]
    pub auth: AuthArgs,

    /// The crawl job ID
    #[arg(value_name = "ID")]
    pub id: String,
}

#[derive(Args, Clone, Debug)]
pub struct CrawlCancelArgs {
    #[command(flatten)]
    pub auth: AuthArgs,

    /// The crawl job ID to cancel
    #[arg(value_name = "ID")]
    pub id: String,
}

// ---------------------------------------------------------------------------------------------
// map

#[derive(Args, Clone, Debug)]
pub struct MapArgs {
    #[command(flatten)]
    pub auth: AuthArgs,

    /// The base URL to map
    #[arg(value_name = "URL")]
    pub url: String,

    /// Query to order results by relevance
    #[arg(long, value_name = "QUERY")]
    pub search: Option<String>,

    /// Sitemap mode: skip, include, only (default: include)
    #[arg(long, value_name = "MODE")]
    pub sitemap: Option<String>,

    /// Include subdomains (default: true)
    #[arg(long, num_args = 0..=1, default_missing_value = "true")]
    pub include_subdomains: Option<bool>,

    /// Exclude URLs with query parameters (default: true)
    #[arg(long, num_args = 0..=1, default_missing_value = "true")]
    pub ignore_query_params: Option<bool>,

    /// Bypass the sitemap cache
    #[arg(long)]
    pub ignore_cache: bool,

    /// Maximum links to return (default: 5000, max: 100000)
    #[arg(long, value_name = "COUNT")]
    pub limit: Option<u32>,

    /// Timeout in milliseconds
    #[arg(long, value_name = "MS")]
    pub timeout: Option<u64>,
}

// ---------------------------------------------------------------------------------------------
// search

#[derive(Args, Clone, Debug)]
pub struct SearchArgs {
    #[command(flatten)]
    pub auth: AuthArgs,

    /// The search query (max 500 characters)
    #[arg(value_name = "QUERY")]
    pub query: String,

    /// Number of results (1-100, default: 10)
    #[arg(long, value_name = "COUNT")]
    pub limit: Option<u32>,

    /// ISO country code (default: US)
    #[arg(long, value_name = "CODE")]
    pub country: Option<String>,

    /// Geo-target location string
    #[arg(long, value_name = "LOCATION")]
    pub location: Option<String>,

    /// Request timeout in milliseconds (default: 60000)
    #[arg(long, value_name = "MS")]
    pub timeout: Option<u64>,

    /// Comma-separated scrape formats: markdown, html, rawHtml, links
    #[arg(long, value_name = "FORMATS")]
    pub formats: Option<String>,

    /// Exclude headers, navigation, footers
    #[arg(long)]
    pub only_main_content: bool,

    /// Emulate a mobile device for scraping
    #[arg(long)]
    pub mobile: bool,
}

// ---------------------------------------------------------------------------------------------
// login

#[derive(Args, Clone, Debug)]
pub struct LoginArgs {
    /// Account name to store (default: "default")
    #[arg(value_name = "NAME", default_value = "default")]
    pub name: String,

    /// Firecrawl API key (prompted for securely if omitted)
    #[arg(long, value_name = "KEY", conflicts_with = "api_key_stdin")]
    pub api_key: Option<String>,

    /// Read the API key from stdin
    #[arg(long)]
    pub api_key_stdin: bool,

    /// Do not open the browser to the API keys page automatically
    #[arg(long)]
    pub no_browser: bool,

    /// Replace the key on an account that already exists
    #[arg(long)]
    pub force: bool,

    /// Store the key without calling the API to check it first
    #[arg(long)]
    pub no_verify: bool,
}

// ---------------------------------------------------------------------------------------------
// accounts

#[derive(Subcommand, Clone, Debug)]
pub enum AccountsCommand {
    /// Add an account and store its API key in the OS keystore
    Add {
        /// Short name for this account, used as --account elsewhere
        name: String,

        /// Firecrawl API key (prompted for without echo if omitted)
        #[arg(long, value_name = "KEY", conflicts_with = "api_key_stdin")]
        api_key: Option<String>,

        /// Read the API key from stdin
        #[arg(long)]
        api_key_stdin: bool,

        /// Replace the key on an account that already exists
        #[arg(long)]
        force: bool,

        /// Store the key without calling the API to check it first
        #[arg(long)]
        no_verify: bool,
    },

    /// List configured accounts
    List {
        /// Call the API once per account instead of reporting stored state
        #[arg(long)]
        check: bool,
    },

    /// Check that an account's stored key still works
    Test {
        /// Account name
        name: String,
    },

    /// Remove an account and delete its stored key
    Remove {
        /// Account name
        name: String,

        /// Skip the confirmation prompt
        #[arg(long)]
        yes: bool,
    },
}
