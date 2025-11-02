use crate::utils::pretty_print;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::GITHUB_RAW_BASE;

#[derive(clap::Args, Debug, Clone)]
pub struct PreviewArgs {
    #[arg(allow_hyphen_values = true)]
    pub templates: Vec<String>,
}

impl super::Runnable for PreviewArgs {
    fn run(&self) -> anyhow::Result<()> {
        if self.templates.is_empty() {
            return Err(anyhow::anyhow!(
                "No issue template specified. Pass template names as arguments."
            ));
        }

        for template_name in &self.templates {
            preview_single_template(template_name)?;
        }

        Ok(())
    }
}

fn preview_single_template(template: &str) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/issue-templates/{}.yml", GITHUB_RAW_BASE, template);

    let pb = progress::spinner(&format!("Fetching issue template: {}", template));
    let content = fetcher.fetch_content(&url)?;
    let msg = format!("Successfully fetched issue template: {}", template);
    pb.set_message(msg);
    pb.finish_and_clear();

    pretty_print::print_highlighted("yml", &content);
    Ok(())
}
