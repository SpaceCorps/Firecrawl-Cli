# Firecrawl CLI

[![Release](https://img.shields.io/github/v/release/SpaceCorps/Firecrawl-Cli?color=orange&label=version)](https://github.com/SpaceCorps/Firecrawl-Cli/releases/latest)
[![CI](https://github.com/SpaceCorps/Firecrawl-Cli/actions/workflows/ci.yml/badge.svg)](https://github.com/SpaceCorps/Firecrawl-Cli/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/badge/docs-online-success)](https://spacecorps.github.io/Firecrawl-Cli/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A blazing fast, native command-line tool and agent interface for the [Firecrawl](https://firecrawl.dev) web scraping and crawling API (v2). Built in Rust for developers and autonomous AI workflows.

---

## Highlights

- ⚡ **Sub-5ms Startup**: Compiled as a standalone native binary with zero runtime dependencies. Executes in ~1–3 ms with no runtime startup delay.
- 🔐 **OS Keystore Integration**: Store and manage API keys securely in your host operating system vault (macOS Keychain, Linux Secret Service, Windows DPAPI) via `firecrawl login` or `firecrawl accounts add`.
- 🌐 **Complete Firecrawl v2 API Surface**: Full coverage for single-page scraping (`scrape`), asynchronous multi-page crawling (`crawl start`, `status`, `cancel`), sitemap mapping (`map`), and web search (`search`).
- 🤖 **AI Agent Native**: Clean YAML default output for human and LLM terminal inspection, deterministic `--json` mode for agent loops and `jq`, and structured error envelopes on `stderr`.
- 🛡️ **Backward Compatible**: Supports direct `--api-key <key>`, `FIRECRAWL_API_KEY` environment variables, and self-hosted Firecrawl instances via `FIRECRAWL_API_URL`.

---

## Installation

### Using Cargo

```bash
cargo install --git https://github.com/SpaceCorps/Firecrawl-Cli --locked
```

### Pre-built Standalone Binaries

Download precompiled standalone binaries directly from [GitHub Releases](https://github.com/SpaceCorps/Firecrawl-Cli/releases/latest):

| Platform | Architecture | Binary Package |
|:---|:---|:---|
| **macOS** | Apple Silicon (`aarch64`) | [`firecrawl-v1.0.0-aarch64-apple-darwin.tar.gz`](https://github.com/SpaceCorps/Firecrawl-Cli/releases/download/v1.0.0/firecrawl-v1.0.0-aarch64-apple-darwin.tar.gz) |
| **macOS** | Intel (`x86_64`) | [`firecrawl-v1.0.0-x86_64-apple-darwin.tar.gz`](https://github.com/SpaceCorps/Firecrawl-Cli/releases/download/v1.0.0/firecrawl-v1.0.0-x86_64-apple-darwin.tar.gz) |
| **Linux** | x86_64 (musl static) | [`firecrawl-v1.0.0-x86_64-unknown-linux-musl.tar.gz`](https://github.com/SpaceCorps/Firecrawl-Cli/releases/download/v1.0.0/firecrawl-v1.0.0-x86_64-unknown-linux-musl.tar.gz) |
| **Linux** | ARM64 (musl static) | [`firecrawl-v1.0.0-aarch64-unknown-linux-musl.tar.gz`](https://github.com/SpaceCorps/Firecrawl-Cli/releases/download/v1.0.0/firecrawl-v1.0.0-aarch64-unknown-linux-musl.tar.gz) |
| **Windows**| x64 (MSVC) | [`firecrawl-v1.0.0-x86_64-pc-windows-msvc.zip`](https://github.com/SpaceCorps/Firecrawl-Cli/releases/download/v1.0.0/firecrawl-v1.0.0-x86_64-pc-windows-msvc.zip) |

---

## Quickstart

### 1. Authenticate

```bash
# Interactive login (opens Firecrawl dashboard in browser, prompts for key, saves to OS keystore)
firecrawl login

# Or set in environment
export FIRECRAWL_API_KEY="fc-your-api-key"

# Or configure a named account
printf %s "$KEY" | firecrawl accounts add production --api-key-stdin
```

### 2. Scrape a Web Page

```bash
# Scrape clean LLM-ready markdown
firecrawl scrape https://example.com

# Scrape multiple formats with mobile emulation
firecrawl scrape https://example.com --formats markdown,html --mobile --only-clean-content
```

### 3. Asynchronous Multi-Page Crawl

```bash
# Start a crawl job
firecrawl crawl start https://docs.rs/clap --limit 100 --max-depth 2

# Check crawl job status and download extracted pages
firecrawl crawl status 550e8400-e29b-41d4-a716-446655440000 --json

# Cancel a crawl job
firecrawl crawl cancel 550e8400-e29b-41d4-a716-446655440000
```

### 4. Sitemap & URL Mapping

```bash
# Discover all URLs on a domain
firecrawl map https://example.com

# Filter and rank by relevance
firecrawl map https://example.com --search "blog" --limit 50
```

### 5. Web Search with Extraction

```bash
# Search and extract markdown contents
firecrawl search "Rust 2024 edition features" --limit 5 --formats markdown
```

---

## Command Reference

| Command | Description | Example |
|:---|:---|:---|
| `firecrawl scrape <URL>` | Scrape single URL and extract clean markdown or HTML | `firecrawl scrape https://example.com --formats markdown` |
| `firecrawl crawl start <URL>` | Start an asynchronous multi-page crawl job | `firecrawl crawl start https://example.com --limit 100` |
| `firecrawl crawl status <ID>` | Retrieve status and scraped pages for crawl job | `firecrawl crawl status <ID> --json` |
| `firecrawl crawl cancel <ID>` | Cancel an active crawl job | `firecrawl crawl cancel <ID>` |
| `firecrawl map <URL>` | Discover indexable URLs from a website / sitemap | `firecrawl map https://example.com --search "docs"` |
| `firecrawl search <QUERY>` | Search the web and extract full page contents | `firecrawl search "query" --limit 5 --formats markdown` |
| `firecrawl login [name]` | Authenticate interactively via browser token flow | `firecrawl login work` |
| `firecrawl accounts add <n>` | Add a named account to local OS keystore | `firecrawl accounts add work --api-key fc-xxx` |
| `firecrawl accounts list` | List configured accounts (pass `--check` to verify) | `firecrawl accounts list --check` |
| `firecrawl accounts test <n>` | Test connectivity and inspect credit balance | `firecrawl accounts test work` |
| `firecrawl accounts remove <n>` | Remove account and delete key from keystore | `firecrawl accounts remove work --yes` |
| `firecrawl agent-readme` | Print embedded operating manual for LLM agents | `firecrawl agent-readme --json` |

---

## Agent Integration & Exit Codes

`firecrawl` outputs structured errors to `stderr` with stable exit codes:

```json
{
  "error": "The Firecrawl API key was rejected.",
  "code": "auth_required",
  "detail": "HTTP 401: Unauthorized",
  "remediation": "firecrawl login"
}
```

| Exit Code | Code Name | Meaning |
|:---|:---|:---|
| `0` | `ok` | Success |
| `1` | `error` | Unclassified error |
| `2` | `network` | Network failure or timeout |
| `3` | `auth_required` | Invalid/missing API key |
| `4` | `not_found` | Resource or job not found |
| `5` | `rate_limited` | Rate limit or credit quota exceeded |
| `6` | `invalid_input` | Parameter or validation error |
| `7` | `no_account` | No account or API key configured |

---

## Credits & License

- Original .NET CLI prototype (`Firecrawl.Console`) by [Niels Bosma](https://github.com/nielsbosma/Firecrawl.Console).
- Re-architected in native Rust (Edition 2024) and maintained by [SpaceCorps](https://github.com/SpaceCorps).
- Released under the [MIT License](LICENSE).
