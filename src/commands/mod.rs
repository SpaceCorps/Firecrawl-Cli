//! Subcommand routing and execution.

pub mod accounts;
pub mod crawl;
pub mod login;
pub mod map;
pub mod scrape;
pub mod search;

use crate::cli::Command;
use crate::error::Result;
use crate::readme;

pub fn run(cmd: Command) -> Result<()> {
    match cmd {
        Command::Scrape(args) => scrape::run(args),
        Command::Crawl(cmd) => crawl::run(cmd),
        Command::Map(args) => map::run(args),
        Command::Search(args) => search::run(args),
        Command::Login(args) => login::run(args),
        Command::Accounts(cmd) => accounts::run(cmd),
        Command::AgentReadme => {
            readme::print();
            Ok(())
        }
    }
}
