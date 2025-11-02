use colored::*;
use std::path::{Path, PathBuf};

use crate::utils::file;
use crate::utils::manifest_navigator::ManifestNavigator;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::GITHUB_RAW_BASE;

const OUTPUT_BASE_PATH: &str = ".github";
const OUTPUT: &str = "ISSUE_TEMPLATE";

// Command to add issue templates

#[derive(clap::Args, Clone, Debug)]
pub struct AddArgs {
    /// Template names to add (e.g., rust, python, global/windows)
    #[arg(value_name = "TEMPLATE")]
    pub templates: Vec<String>,

    /// Directory to save the issue templates
    #[arg(long, value_name = "DIR")]
    pub dir: Option<PathBuf>,

    /// Force overwrite existing issue template files
    #[arg(long)]
    pub force: bool,

    /// Download all available templates
    #[arg(long)]
    pub all: bool,

    /// Output file names for the templates (in order of templates)
    #[arg(short='o', long, value_name = "OUTPUT", num_args = 1.., requires = "templates")]
    pub output: Vec<String>,
}

impl super::Runnable for AddArgs {
    fn run(&self) -> anyhow::Result<()> {
        if self.all {
            download_all_templates(self.dir.as_ref(), self.force)?;
        } else if self.templates.is_empty() {
            return Err(anyhow::anyhow!(
                "No issue template specified. Use `--all` or pass template names."
            ));
        } else {
            if !self.output.is_empty() {
                if self.templates.len() != self.output.len() {
                    return Err(anyhow::anyhow!(
                        "The number of templates and output file names must match."
                    ));
                }
                for (template_name, output_name) in self.templates.iter().zip(self.output.iter()) {
                    download_single_template(
                        template_name,
                        self.dir.as_ref(),
                        self.force,
                        Some(output_name.clone()),
                    )?;
                }
            } else {
                for template_name in &self.templates {
                    download_single_template(template_name, self.dir.as_ref(), self.force, None)?;
                }
            }
        }

        Ok(())
    }
}

// Helper functions
fn download_all_templates(dir_path: Option<&PathBuf>, force: bool) -> anyhow::Result<()> {
    let manifest_url = format!("{}/issue-templates/manifest.yml", GITHUB_RAW_BASE);
    let manifest_navigator = ManifestNavigator::new(&manifest_url)?;
    let template_entries = manifest_navigator.list_entries()?;

    let mut errors = Vec::new();

    for entry in template_entries {
        // Remove extension from name for consistency
        let template_name = match entry.name.rfind('.') {
            Some(idx) => &entry.name[..idx],
            None => &entry.name,
        };

        if let Err(e) = download_single_template(template_name, dir_path, force, None) {
            eprintln!(
                "{} Failed to add template '{}': {}",
                "✗".red(),
                template_name,
                e
            );
            errors.push((template_name.to_string(), e));
        }
    }

    let default_output = format!("{}/{}", OUTPUT_BASE_PATH, OUTPUT);
    let output_location = dir_path
        .map(|p| p.display().to_string())
        .unwrap_or(default_output);

    if errors.is_empty() {
        println!(
            "{} Downloaded all issue templates to {}",
            "✓".green(),
            output_location
        );
    } else {
        println!(
            "{} Some templates failed to download. See errors above.",
            "⚠".yellow()
        );
    }

    Ok(())
}

fn download_single_template(
    template_name: &str,
    dir_path: Option<&PathBuf>,
    force: bool,
    output: Option<String>,
) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    // If template_name does not have an extension, append ".yml"
    let template_file = if Path::new(template_name).extension().is_some() {
        template_name.to_string()
    } else {
        format!("{}.yml", template_name)
    };
    let url = format!("{}/issue-templates/{}", GITHUB_RAW_BASE, template_file);

    let msg = format!("Downloading issue template: {}", template_name);
    let pb = progress::spinner(&msg);
    let content = fetcher.fetch_content(&url)?;
    pb.set_message("Download Complete");
    pb.finish_and_clear();

    // Determine output path logic
    let dest_path = if let Some(mut output_file) = output {
        // Technical Debt: Add .yml extension if not present. Not flexible for other formats.
        if Path::new(&output_file).extension().is_none() {
            output_file.push_str(".yml");
        }
        dir_path
            .map(|p| p.join(&output_file))
            .unwrap_or_else(|| Path::new(OUTPUT_BASE_PATH).join(OUTPUT).join(&output_file))
    } else {
        // Default: .github/ISSUE_TEMPLATE/<template_name>.yml
        let default_path = Path::new(OUTPUT_BASE_PATH).join(OUTPUT);
        dir_path
            .map(|p| p.join(format!("{}", template_file)))
            .unwrap_or_else(|| default_path.join(format!("{}", template_file)))
    };

    file::save_file(&content, &dest_path, force)?;

    Ok(())
}
