---
title: "Authentication Guide"
description: "Authentication methods, OS keystore credential storage, and error handling for developers and AI agents using the Firecrawl CLI."
author: "SpaceCorps"
date: "2026-09-24"
---

# Authentication Guide for Firecrawl CLI

This document outlines authentication methods, credential storage, and error handling for developers and autonomous AI agents using the Firecrawl CLI.

## Overview
The Firecrawl CLI communicates with the Firecrawl v2 API (`https://api.firecrawl.dev/v2/`). Authentication uses Bearer tokens (`fc-...`) generated in the Firecrawl dashboard. Tokens can be stored in the host operating system's native keystore, passed as command-line flags, or supplied via environment variables.

## Prerequisites
- A Firecrawl account ([firecrawl.dev](https://www.firecrawl.dev))
- An API key generated from the API keys page (`https://www.firecrawl.dev/app/api-keys`)
- Firecrawl CLI installed on your machine (`cargo install --git https://github.com/SpaceCorps/Firecrawl-Cli --locked`)

## Authentication Methods

### 1. Interactive Login (`firecrawl login`)
The recommended approach for interactive developer workstations:
```bash
firecrawl login [account_name]
```
1. The CLI launches your default web browser directly to `https://www.firecrawl.dev/app/api-keys`.
2. Copy your Firecrawl API key (`fc-...`).
3. Paste the key into the terminal prompt (masked without terminal echo).
4. The CLI validates the key by calling the Firecrawl team usage endpoint.
5. The API key is securely encrypted in the OS keystore (macOS Keychain, Windows DPAPI, or Linux Secret Service) under the account name (defaults to `default`).

### 2. Multi-Account Keystore (`firecrawl accounts add`)
Add named accounts for distinct environments, projects, or client organizations:
```bash
# Add with interactive secure prompt
firecrawl accounts add staging

# Pipe key from standard input (prevents shell history leakage)
printf %s "$STAGING_KEY" | firecrawl accounts add staging --api-key-stdin

# Replace existing account credentials
firecrawl accounts add staging --api-key "$NEW_KEY" --force
```

### 3. Environment Variable (`FIRECRAWL_API_KEY`)
For CI/CD pipelines, containerized agents, and headless environments where no keystore daemon is available:
```bash
export FIRECRAWL_API_KEY="fc-your-api-key"
firecrawl scrape https://example.com
```

### 4. Direct Command Flag (`--api-key`)
Pass the key per invocation:
```bash
firecrawl scrape https://example.com --api-key "$FIRECRAWL_API_KEY"
```

## Account Inspection & Verification
List and test stored accounts:
```bash
firecrawl accounts list              # lists stored accounts and secret store type
firecrawl accounts list --check      # tests API connectivity for each account
firecrawl accounts test <name>       # checks validity and reports remaining credits
firecrawl accounts remove <name> -y  # removes credentials from local keystore
```

## Self-Hosted Firecrawl
To point the CLI at a self-hosted Firecrawl instance, configure `FIRECRAWL_API_URL`:
```bash
export FIRECRAWL_API_URL="http://localhost:3002/v2/"
firecrawl scrape http://example.local
```

## Error Codes
When authentication fails, commands exit with standard machine-readable exit codes and error envelopes:
- `auth_required` (exit code 3): API key invalid, revoked, or insufficient plan permissions.
- `no_account` (exit code 7): No account or key found.
- `rate_limited` (exit code 5): Credit limit or concurrency limit exceeded.
