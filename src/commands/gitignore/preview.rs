use crate::utils::cache::CacheManager;
use crate::utils::pretty_print;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::{ensure_gitignore_cache, find_template_in_cache, GITHUB_RAW_BASE};

#[derive(clap::Args, Debug, Clone)]
pub struct PreviewArgs {
    /// Template names to preview (e.g., rust, python, global/windows)
    #[arg()]
    pub args: Vec<String>,

    /// Update the gitignore cache
    #[arg(long = "update-cache")]
    pub update_cache: bool,

    /// Disable colored output
    #[arg(long = "no-color")]
    pub no_color: bool,
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
            preview_single_template(template_name, &cache, self.no_color)?;
        }

        Ok(())
    }
}

fn preview_single_template(
    template: &str,
    cache: &super::Cache<String>,
    no_color: bool,
) -> anyhow::Result<()> {
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

    if no_color {
        println!("{}", content);
    } else {
        pretty_print::print_highlighted("gitignore", &content);
    }
    Ok(())
}
