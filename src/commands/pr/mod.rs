use clap::Subcommand;

use crate::commands::base::Runnable;

pub mod add;
pub mod list;
pub mod preview;

// Global constants - these can stay in the main module file
const GITHUB_RAW_BASE: &str = "https://raw.githubusercontent.com/Byte-Barn/gitcraft/main/templates";

#[derive(Subcommand)]
pub enum Command {
    /// Add one or more PR templates to the repository
    Add(add::AddArgs),
    /// List available PR templates
    List(list::ListArgs),
    /// Preview a specific PR template
    Preview(preview::PreviewArgs),
}

impl Command {
    pub fn execute(&self) -> anyhow::Result<()> {
        match self {
            Command::Add(args) => args.run(),
            Command::List(args) => args.run(),
            Command::Preview(args) => args.run(),
        }
    }
}
