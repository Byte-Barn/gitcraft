use crate::utils::pretty_print;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::GITHUB_RAW_BASE;

#[derive(clap::Args, Debug, Clone)]
pub struct PreviewArgs {
    #[arg(help = "PR template names to preview")]
    pub args: Vec<String>,

    /// Disable colored output
    #[arg(long = "no-color")]
    pub no_color: bool,
}

impl super::Runnable for PreviewArgs {
    fn run(&self) -> anyhow::Result<()> {
        if self.args.is_empty() {
            return Err(anyhow::anyhow!(
                "No PR template specified. Pass template names to preview."
            ));
        }

        for template_name in &self.args {
            preview_single_template(template_name, self.no_color)?;
        }

        Ok(())
    }
}

fn preview_single_template(template: &str, no_color: bool) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/pr-templates/{}.md", GITHUB_RAW_BASE, template);

    let pb = progress::spinner(&format!("Fetching PR template: {}", template));
    let content = fetcher.fetch_content(&url)?;
    let msg = format!("Successfully fetched PR template: {}", template);
    pb.set_message(msg);
    pb.finish_and_clear();

    println!("\n        === Preview: {} === \n", template);
    if no_color {
        println!("{}", content);
    } else {
        pretty_print::print_highlighted("md", &content);
    }
    Ok(())
}
