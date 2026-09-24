//! Multi-account resolution and credential access.
//!
//! An API key is obtained either via an explicit `--account <name>` stored in the native OS
//! keystore, or via direct `--api-key <key>` / `FIRECRAWL_API_KEY` environment variable.

use clap::Args;
use serde_json::Value;

use crate::client::Client;
use crate::config::{self, Config};
use crate::error::{Error, ErrorCode, Result};
use crate::secrets;

#[derive(Args, Clone, Debug, Default)]
pub struct AuthArgs {
    /// Account to run against (see 'firecrawl accounts list')
    #[arg(short = 'a', long, value_name = "ACCOUNT")]
    pub account: Option<String>,

    /// Firecrawl API key (or set FIRECRAWL_API_KEY env var)
    #[arg(long, value_name = "KEY")]
    pub api_key: Option<String>,
}

pub struct Resolved {
    #[allow(dead_code)]
    pub name: String,
    pub api_key: String,
}

impl Resolved {
    pub fn client(&self) -> Client {
        Client::new(&self.api_key)
    }
}

pub fn resolve(auth: &AuthArgs) -> Result<Resolved> {
    let config = config::load()?;

    // 1. Explicit account name requested
    if let Some(requested) = auth.account.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        let Some((name, _account)) = config.find(requested) else {
            return Err(Error::new(ErrorCode::NoAccount, format!("No account named '{requested}'."))
                .detail(describe(&config))
                .fix("firecrawl accounts list"));
        };

        let key = secrets::store()?.get(&secrets::account_key(name))?;
        let Some(api_key) = key.filter(|k| !k.trim().is_empty()) else {
            return Err(Error::new(ErrorCode::AuthRequired, format!("Account '{name}' has no stored API key."))
                .detail("The config entry exists but the keystore has nothing under it.")
                .fix(format!("firecrawl accounts add {name} --api-key <key>")));
        };

        return Ok(Resolved { name: name.clone(), api_key });
    }

    // 2. Direct API key passed via flag or FIRECRAWL_API_KEY
    let env_key = std::env::var("FIRECRAWL_API_KEY").ok();
    let direct_key = auth.api_key.as_deref().or(env_key.as_deref()).map(str::trim).filter(|s| !s.is_empty());
    if let Some(key) = direct_key {
        return Ok(Resolved { name: "direct".into(), api_key: key.to_string() });
    }

    // 3. Fallback when neither is specified
    if config.accounts.is_empty() {
        Err(Error::new(
            ErrorCode::NoAccount,
            "No account or API key configured. Use --api-key, set FIRECRAWL_API_KEY, or run 'firecrawl login'.",
        )
        .detail(describe(&config))
        .fix("firecrawl login"))
    } else {
        Err(Error::new(ErrorCode::NoAccount, "No account specified. Pass --account <name> or --api-key <key>.")
            .detail(describe(&config))
            .fix("firecrawl accounts list"))
    }
}

pub fn describe(config: &Config) -> String {
    if config.accounts.is_empty() {
        return "No accounts are configured yet. Run 'firecrawl accounts add <name> --api-key <key>' or 'firecrawl login'.".into();
    }
    let listed: Vec<String> = config
        .sorted()
        .into_iter()
        .map(|(k, v)| if v.identity.trim().is_empty() { k.clone() } else { format!("{k} ({})", v.identity) })
        .collect();
    format!("Configured accounts: {}", listed.join(", "))
}

pub mod identity {
    use super::*;

    /// Inspects the key against Firecrawl team/credit-usage to extract an identity label.
    pub fn probe(client: &Client, api_key: &str) -> String {
        if let Ok(usage) = client.get("team/credit-usage") {
            if let Some(credits) = usage.get("remaining_credits").or_else(|| usage.get("remainingCredits")) {
                return format!("credits: {credits}");
            }
            if let Some(team_id) = usage.get("team_id").or_else(|| usage.get("teamId")).and_then(Value::as_str) {
                return format!("team: {team_id}");
            }
        }
        mask_key(api_key)
    }

    fn mask_key(key: &str) -> String {
        let key = key.trim();
        if key.len() <= 8 { "key: ****".to_string() } else { format!("key: {}...{}", &key[..4], &key[key.len() - 4..]) }
    }
}
