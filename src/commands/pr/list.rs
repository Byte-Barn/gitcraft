use colored::*;

use crate::utils::get_comment;
use crate::utils::manifest_navigator::ManifestNavigator;
use crate::utils::remote::Fetcher;

use super::GITHUB_RAW_BASE;

#[derive(clap::Args, Clone, Debug)]
pub struct ListArgs {
    // You can add options here if needed in the future
}

impl super::Runnable for ListArgs {
    fn run(&self) -> anyhow::Result<()> {
        list_all_pr_templates()
    }
}

fn list_all_pr_templates() -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    let manifest_url = format!("{}/pr-templates/manifest.yml", GITHUB_RAW_BASE);
    let manifest_navigator = ManifestNavigator::new(&manifest_url)?;
    let template_entries = manifest_navigator.list_entries()?;

    if template_entries.is_empty() {
        println!("No pull request templates found.");
    } else {
        println!("{} Available pull request templates:", "✓".green());
        for entry in template_entries {
            let file_url = &entry.full_url;
            let extension = std::path::Path::new(file_url)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");
            let comment = match fetcher.fetch_content(file_url) {
                Ok(text) => text
                    .lines()
                    .next()
                    .and_then(|line| get_comment::extract_comment(line, extension)),
                _ => None,
            };

            match comment {
                Some(description) => {
                    println!("  {} {:<12} - {}", ">".green(), entry.name, description)
                }
                None => println!("  {}", entry.name),
            }
        }
    }
    Ok(())
}
