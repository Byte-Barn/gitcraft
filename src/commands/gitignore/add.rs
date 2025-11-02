use std::path::{Path, PathBuf};

use anyhow::Result;
use colored::*;

use crate::utils::cache::{Cache, CacheManager};
use crate::utils::file;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::{
    GITHUB_RAW_BASE, OUTPUT, OUTPUT_BASE_PATH, ensure_gitignore_cache, find_template_in_cache,
};

#[derive(clap::Args, Clone, Debug)]
pub struct AddArgs {
    /// Template names to add (e.g., rust, python, global/windows)
    #[arg(value_name = "TEMPLATE")]
    pub templates: Vec<String>,

    /// Directory to save the .gitignore file
    #[arg(long, value_name = "DIR")]
    pub dir: Option<PathBuf>,

    /// Force overwrite existing .gitignore file
    #[arg(long)]
    pub force: bool,

    /// Download all available templates
    #[arg(long)]
    pub all: bool,

    /// Append to the existing .gitignore file instead of overwriting
    #[arg(long, short = 'a')]
    pub append: bool,

    /// Update the gitignore cache
    #[arg(long = "update-cache", default_value = "false")]
    pub update_cache: bool,

    /// Use the remote template file name as the output file name
    #[arg(long = "use-remote-name", short = 'n', default_value_t = false)]
    pub use_remote_name: bool,

    /// Output file name (default: .gitignore)
    #[arg(
        long = "output",
        short = 'o',
        value_name = "FILENAME",
        num_args = 1.., // Accepts one or more values
        default_value = ".gitignore",
        requires = "templates"
    )]
    pub output: Vec<String>,
}

impl super::Runnable for AddArgs {
    fn run(&self) -> anyhow::Result<()> {
        let mut cache_manager = CacheManager::new()?;

        let cache: Cache<String> = ensure_gitignore_cache(&mut cache_manager, self.update_cache)?;

        let dir = match &self.dir {
            Some(dir) => Some(dir.clone()),
            None => Some(file::find_repo_root()?),
        };

        if self.all {
            download_all_templates(dir.as_ref(), self.force, &cache, self.use_remote_name)?;
        } else if self.templates.is_empty() {
            return Err(anyhow::anyhow!(
                "No gitignore template specified. Use `--all` or pass template names."
            ));
        } else {
            download_templates(
                &self.templates,
                dir.as_ref(),
                &self.output,
                self.force,
                &cache,
                self.append,
                self.use_remote_name,
            )?;
        }

        Ok(())
    }
}

fn download_all_templates(
    dir_path: Option<&PathBuf>,
    force: bool,
    cache: &Cache<String>,
    use_remote_name: bool,
) -> Result<()> {
    println!("Fetching all gitignore templates...");
    let fetcher = Fetcher::new();

    if use_remote_name {
        // Save each template as its remote filename (e.g., Python.gitignore)
        for (_key, rel_path_entry) in cache.entries.iter() {
            let url = format!("{}/{}", GITHUB_RAW_BASE, rel_path_entry.data);

            // Extract remote filename from rel_path_entry.data
            let remote_filename = Path::new(&rel_path_entry.data)
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid template path: {}", rel_path_entry.data))?;

            let dest_path = dir_path
                .map(|p| p.join(remote_filename))
                .unwrap_or_else(|| {
                    Path::new(OUTPUT_BASE_PATH)
                        .join(OUTPUT)
                        .join(remote_filename)
                });

            if force && dest_path.exists() {
                std::fs::remove_file(&dest_path)?;
            }

            let msg = format!("Downloading gitignore template: {}", remote_filename);
            let pb = progress::spinner(&msg);
            let content = fetcher.fetch_content(&url)?;
            pb.set_message("Download Complete");
            pb.finish_and_clear();

            let section = format!("# ===== {} =====\n{}\n\n", remote_filename, content);
            file::save_file(&section, &dest_path, force)?;
        }
    } else {
        // Merge all templates into a single .gitignore file
        let dest_path = dir_path
            .map(|p| p.join(".gitignore"))
            .unwrap_or_else(|| Path::new(OUTPUT_BASE_PATH).join(OUTPUT).join(".gitignore"));

        if force && dest_path.exists() {
            std::fs::remove_file(&dest_path)?;
        }

        for (key, rel_path_entry) in cache.entries.iter() {
            let url = format!("{}/{}", GITHUB_RAW_BASE, rel_path_entry.data);

            let msg = format!("Downloading gitignore template: {}", key);
            let pb = progress::spinner(&msg);
            let content = fetcher.fetch_content(&url)?;
            pb.set_message("Download Complete");
            pb.finish_and_clear();

            let section = format!("# ===== {}.gitignore =====\n{}\n\n", key, content);
            file::append_file(&section, &dest_path, None)?;
        }

        println!(
            "{} Downloaded and merged all gitignore templates to {}",
            "✓".green(),
            dest_path.display()
        );
    }

    Ok(())
}

fn download_templates(
    templates: &[String],
    dir_path: Option<&PathBuf>,
    output: &[String],
    force: bool,
    cache: &Cache<String>,
    append: bool,
    use_remote_name: bool,
) -> Result<()> {
    let fetcher = Fetcher::new(); // Create once, reuse

    if use_remote_name {
        // Each template is saved using its remote filename (e.g., Python.gitignore)
        for template_name in templates {
            let template_path = find_template_in_cache(template_name, cache)?;
            let url = format!("{}/{}", GITHUB_RAW_BASE, template_path);

            // Extract filename from template_path (e.g., "Python.gitignore")
            let remote_filename = Path::new(&template_path)
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid template path: {}", template_path))?;

            let msg = format!("Downloading gitignore template: {}", template_name);
            let pb = progress::spinner(&msg);
            let content = fetcher.fetch_content(&url)?;
            pb.set_message("Download Complete");
            pb.finish_and_clear();

            let section = format!("# ===== {} =====\n{}\n\n", remote_filename, content);

            let dest_path = dir_path
                .map(|p| p.join(remote_filename))
                .unwrap_or_else(|| {
                    Path::new(OUTPUT_BASE_PATH)
                        .join(OUTPUT)
                        .join(remote_filename)
                });

            if append {
                file::append_file(&section, &dest_path, None)?;
            } else {
                file::save_file(&section, &dest_path, force)?;
            }
        }
    } else if output.len() == templates.len() {
        // Save each template to its own file as specified in output
        for (template_name, output_file) in templates.iter().zip(output.iter()) {
            // check if the template_name has  a .gitignore ext rm it to normalize it
            let template_name = template_name.strip_suffix(".gitignore").unwrap_or(template_name);
            let template_path = find_template_in_cache(template_name, cache)?;
            let url = format!("{}/{}", GITHUB_RAW_BASE, template_path);

            let msg = format!("Downloading gitignore template: {}", template_name);
            let pb = progress::spinner(&msg);
            let content = fetcher.fetch_content(&url)?;
            pb.set_message("Download Complete");
            pb.finish_and_clear();

            let section = format!("# ===== {}.gitignore =====\n{}\n\n", template_name, content);

            let dest_path = dir_path
                .map(|p| p.join(output_file))
                .unwrap_or_else(|| Path::new(OUTPUT_BASE_PATH).join(OUTPUT).join(output_file));

            if append {
                file::append_file(&section, &dest_path, None)?;
            } else {
                file::save_file(&section, &dest_path, force)?;
            }

            println!(
                "{} Added gitignore template: {} to {}",
                "✓".green(),
                template_name,
                dest_path.display()
            );
        }
    } else if output.len() == 1 {
        // Merge all templates into one file, but skip invalid templates and collect errors
        let mut merged_content = String::new();
        let mut errors = Vec::new();

        for template_name in templates {
            match find_template_in_cache(template_name, cache) {
                Ok(template_path) => {
                    let url = format!("{}/{}", GITHUB_RAW_BASE, template_path);

                    let msg = format!("Downloading gitignore template: {}", template_name);
                    let pb = progress::spinner(&msg);
                    match fetcher.fetch_content(&url) {
                        Ok(content) => {
                            pb.set_message("Download Complete");
                            pb.finish_and_clear();
                            merged_content.push_str(&format!(
                                "# ===== {}.gitignore =====\n{}\n\n",
                                template_name, content
                            ));
                        }
                        Err(e) => {
                            pb.finish_and_clear();
                            errors.push(format!(
                                "Failed to fetch template '{}': {}",
                                template_name, e
                            ));
                        }
                    }
                }
                Err(e) => {
                    errors.push(format!("Template '{}' not found: {}", template_name, e));
                }
            }
        }

        let dest_path = dir_path
            .map(|p| p.join(&output[0]))
            .unwrap_or_else(|| Path::new(OUTPUT_BASE_PATH).join(OUTPUT).join(&output[0]));

        if !merged_content.is_empty() {
            if append {
                file::append_file(&merged_content, &dest_path, None)?;
            } else {
                file::save_file(&merged_content, &dest_path, force)?;
            }
        }

        // Print errors for invalid templates
        for error in errors {
            eprintln!("{}", error.red());
        }
    } else {
        return Err(anyhow::anyhow!(
            "Number of output files must be either 1 or match the number of templates when not using --use-remote-name"
        ));
    }

    Ok(())
}
