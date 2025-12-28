use crate::utils::pretty_print;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::GITHUB_RAW_BASE;

#[derive(clap::Args, Debug, Clone)]
pub struct PreviewArgs {
    #[arg(allow_hyphen_values = true)]
    pub templates: Vec<String>,

    /// Disable colored output
    #[arg(long = "no-color")]
    pub no_color: bool,
}

impl super::Runnable for PreviewArgs {
    fn run(&self) -> anyhow::Result<()> {
        if self.templates.is_empty() {
            return Err(anyhow::anyhow!(
                "No issue template specified. Pass template names as arguments."
            ));
        }

        for template_name in &self.templates {
            preview_single_template(template_name, self.no_color)?;
        }

        Ok(())
    }
}

fn preview_single_template(template: &str, no_color: bool) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/issue-templates/{}.yml", GITHUB_RAW_BASE, template);

    let pb = progress::spinner(&format!("Fetching issue template: {}", template));
    let content = fetcher.fetch_content(&url)?;
    let msg = format!("Successfully fetched issue template: {}", template);
    pb.set_message(msg);
    pb.finish_and_clear();

    if no_color {
        println!("{}", content);
    } else {
        pretty_print::print_highlighted("yml", &content);
    }
    Ok(())
}
