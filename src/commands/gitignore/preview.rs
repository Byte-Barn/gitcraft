use crate::utils::cache::CacheManager;
use crate::utils::pretty_print;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::{GITHUB_RAW_BASE, ensure_gitignore_cache, find_template_in_cache};

#[derive(clap::Args, Debug, Clone)]
pub struct PreviewArgs {
    /// Template names to preview (e.g., rust, python, global/windows)
    #[arg()]
    pub args: Vec<String>,

    /// Update the gitignore cache
    #[arg(long = "update-cache")]
    pub update_cache: bool,
}

impl super::Runnable for PreviewArgs {
    fn run(&self) -> anyhow::Result<()> {
        if self.args.is_empty() {
            return Err(anyhow::anyhow!(
                "No gitignore template specified. Pass template names as arguments."
            ));
        }

        let mut cache_manager = CacheManager::new()?;
        let cache = ensure_gitignore_cache(&mut cache_manager, self.update_cache)?;

        for template_name in &self.args {
            preview_single_template(template_name, &cache)?;
        }

        Ok(())
    }
}

fn preview_single_template(template: &str, cache: &super::Cache<String>) -> anyhow::Result<()> {
    // normalize template if it has the .gitignore ext
    let template = template.strip_suffix(".gitignore").unwrap_or(template);
    
    // Find the template path in cache
    let template_path = find_template_in_cache(template, cache)?;

    let fetcher = Fetcher::new();
    let url = format!("{}/{}", GITHUB_RAW_BASE, template_path);

    let pb = progress::spinner(&format!("Fetching gitignore template: {}", template));
    let content = fetcher.fetch_content(&url)?;
    let msg = format!("Successfully fetched gitignore template: {}", template);
    pb.set_message(msg);
    pb.finish_and_clear();

    println!("\n        === Preview: {} === \n", template);
    pretty_print::print_highlighted("gitignore", &content);
    Ok(())
}
