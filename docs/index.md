---
title: "Firecrawl CLI"
description: "A blazing fast native command-line tool and agent interface for the Firecrawl v2 API. Built in Rust for developers and autonomous AI agents."
author: "SpaceCorps"
date: "2026-09-24"
canonical: "https://spacecorps.github.io/Firecrawl-Cli/index.md"
---

# Firecrawl CLI

A blazing fast native command-line tool and agent interface for the Firecrawl v2 API. Built in Rust for developers and autonomous AI agents.

## Quickstart

```bash
# Authenticate interactively via browser token flow
firecrawl login

# Or pass your API key directly via environment variable
export FIRECRAWL_API_KEY=fc-your-api-key

# Scrape a web page to clean markdown
firecrawl scrape https://example.com

# Start an asynchronous crawl
firecrawl crawl start https://example.com --limit 50 --max-depth 2

# Map indexable URLs from a website
firecrawl map https://example.com --search "docs"

# Search the web with full page markdown extraction
firecrawl search "Rust web scrapers" --limit 5 --formats markdown
```

## Features

- **Blazing Fast Native Rust**: Sub-5ms startup times with zero runtime dependencies.
- **AI Agent Native**: Clean YAML default output, structured JSON (`--json`) mode, and standardized machine-readable error envelopes on `stderr`.
- **Secure Keystore Integration**: Secrets stored in native macOS Keychain, Windows DPAPI, or Linux Secret Service with zero plain-text leaks.
- **Multi-Account Workspaces**: Isolate testing, production, and client accounts safely.
- **Complete Firecrawl v2 Coverage**: `scrape`, `crawl` (`start`, `status`, `cancel`), `map`, and `search`.

## When to Use This CLI

Use the `firecrawl` CLI whenever you need to:
- Convert any URL to clean LLM-ready markdown or structured HTML.
- Execute multi-page website crawls without managing headless Chromium clusters.
- Map and inspect the link topology of web domains from sitemaps.
- Execute deep web searches that return both search snippets and full extracted content.
- Automate research pipelines within LLM agent tool loops using stable exit codes.

## Documentation Links

- [llms.txt](https://spacecorps.github.io/Firecrawl-Cli/llms.txt)
- [Full Agent Manual](https://spacecorps.github.io/Firecrawl-Cli/llms-full.txt)
- [Pricing](https://spacecorps.github.io/Firecrawl-Cli/pricing.md)
- [Authentication Guide](https://spacecorps.github.io/Firecrawl-Cli/auth.md)
- [About](https://spacecorps.github.io/Firecrawl-Cli/about.html)
- [Contact & Support](https://spacecorps.github.io/Firecrawl-Cli/contact.html)
- [Privacy Policy](https://spacecorps.github.io/Firecrawl-Cli/privacy.html)
- [GitHub Repository](https://github.com/SpaceCorps/Firecrawl-Cli)
