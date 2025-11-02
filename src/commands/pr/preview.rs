use crate::utils::pretty_print;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::GITHUB_RAW_BASE;

#[derive(clap::Args, Debug, Clone)]
pub struct PreviewArgs {
    #[arg(help = "PR template names to preview")]
    pub args: Vec<String>,
}

impl super::Runnable for PreviewArgs {
    fn run(&self) -> anyhow::Result<()> {
        if self.args.is_empty() {
            return Err(anyhow::anyhow!(
                "No PR template specified. Pass template names to preview."
            ));
        }

        for template_name in &self.args {
            preview_single_template(template_name)?;
        }

        Ok(())
    }
}

fn preview_single_template(template: &str) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/pr-templates/{}.md", GITHUB_RAW_BASE, template);

    let pb = progress::spinner(&format!("Fetching PR template: {}", template));
    let content = fetcher.fetch_content(&url)?;
    let msg = format!("Successfully fetched PR template: {}", template);
    pb.set_message(msg);
    pb.finish_and_clear();

    pretty_print::print_highlighted("md", &content);
    Ok(())
}
