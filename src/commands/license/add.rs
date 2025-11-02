use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::{Result, anyhow};
use colored::*;
use regex::Regex;

use crate::utils::cache::CacheManager;
use crate::utils::file;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::{
    SPDX_CACHE_NAME, SPDX_LICENSE_DETAILS_BASE_URL, SPDX_LICENSE_LIST_URL,
    ensure_spdx_license_cache,
};

// Command to add licenses
#[derive(clap::Args, Debug, Clone)]
pub struct AddArgs {
    /// License IDs to add (e.g., mit, apache-2.0)
    #[arg(value_name = "LICENSE")]
    pub licenses: Vec<String>,

    /// Directory to save the license file
    #[arg(long, value_name = "DIR")]
    pub dir: Option<PathBuf>,

    /// Force overwrite existing license file
    #[arg(long)]
    pub force: bool,

    /// Download all available licenses
    #[arg(long)]
    pub all: bool,

    /// Interactive mode for filling placeholders
    #[arg(long, short = 'i')]
    pub interactive: bool,

    /// update the cache
    #[arg(long)]
    pub update_cache: bool,

    /// Additional parameters for license placeholders (key=value format)
    #[arg(long = "param", value_name = "KEY=VALUE", num_args = 0.., action = clap::ArgAction::Append)]
    pub params: Vec<String>,

    /// Output file names for the licenses (in order of licenses)
    #[arg(short = 'o', long, value_name = "OUTPUT", num_args = 1.., requires = "licenses")]
    pub output: Vec<String>,
}

impl super::Runnable for AddArgs {
    fn run(&self) -> Result<()> {
        // Determine the directory to use
        let dir = match &self.dir {
            Some(d) => d.clone(),
            None => file::find_repo_root().unwrap_or_else(|_| PathBuf::from(".")),
        };

        // if update_cache is set, update the license cache
        if self.update_cache {
            let cache_manager = CacheManager::new()?;
            cache_manager.clear_cache(SPDX_CACHE_NAME)?;
        }

        // Parse parameters into a HashMap
        let mut placeholder_params = HashMap::new();
        for param in &self.params {
            if let Some((key, value)) = param.split_once('=') {
                placeholder_params.insert(key.trim().to_lowercase(), value.trim().to_string());
            } else {
                return Err(anyhow!(
                    "Invalid parameter format: '{}'. Use KEY=VALUE",
                    param
                ));
            }
        }

        let config = LicenseDownloadConfig {
            dir_path: Some(&dir),
            force: self.force,
            interactive: self.interactive,
            placeholder_params: &placeholder_params,
            update_cache: self.update_cache,
        };

        if self.all {
            download_all_licenses(&config)?;
        } else if self.licenses.is_empty() {
            return Err(anyhow!(
                "At least one license ID is required (or use --all)"
            ));
        } else {
            if !self.output.is_empty() {
                if self.output.len() != self.licenses.len() {
                    return Err(anyhow!(
                        "Number of output files must match number of licenses"
                    ));
                }

                for (license_id, output_name) in self.licenses.iter().zip(self.output.iter()) {
                    if let Err(e) =
                        download_single_license(license_id, &config, Some(output_name.clone()))
                    {
                        eprintln!(
                            "{}",
                            format!("Failed to download {}: {}", license_id, e).red()
                        );
                    }
                }
            } else {
                for license_id in &self.licenses {
                    if let Err(e) = download_single_license(license_id, &config, None) {
                        eprintln!(
                            "{}",
                            format!("Failed to download {}: {}", license_id, e).red()
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

// Helper functions

// ------------ HANDLE DOWNLOADS ------------
pub struct LicenseDownloadConfig<'a> {
    pub dir_path: Option<&'a PathBuf>,
    pub force: bool,
    pub interactive: bool,
    pub placeholder_params: &'a HashMap<String, String>,
    pub update_cache: bool,
}

fn download_single_license(
    id: &str,
    config: &LicenseDownloadConfig,
    output_filename: Option<String>,
) -> Result<()> {
    let fetcher = Fetcher::new();

    let mut cache_manager = CacheManager::new()?;

    let license_cache = ensure_spdx_license_cache(&mut cache_manager, config.update_cache)?;

    let normalized_id = {
        let id_lower = id.to_lowercase();
        license_cache
            .entries
            .iter()
            .find(|(k, _)| k.to_lowercase() == id_lower)
            .map(|(key, _)| key.clone())
            .ok_or_else(|| {
                anyhow!(
                    "License '{}' not found in SPDX cache. Please check the license ID.",
                    id
                )
            })?
    };

    let details_url = format!("{}/{}.json", SPDX_LICENSE_DETAILS_BASE_URL, normalized_id);
    let pb = progress::spinner(&format!("Fetching license details: {}", id));

    let license_details = fetcher.fetch_json(&details_url).map_err(|e| {
        anyhow!(
            "Failed to fetch license '{}'. This might not be a valid SPDX license ID. Error: {}",
            id,
            e
        )
    })?;

    pb.set_message("Processing license text");

    let license_text = license_details
        .get("licenseText")
        .and_then(|t| t.as_str())
        .ok_or_else(|| anyhow!("License text not found in SPDX data"))?;

    pb.finish_and_clear();

    let processed_text =
        process_placeholders(license_text, config.interactive, config.placeholder_params)?;

    let dest_filename = output_filename.unwrap_or_else(|| "LICENSE".to_string());
    let dest_path: PathBuf = match config.dir_path {
        Some(dir) => dir.join(dest_filename),
        None => PathBuf::from(&dest_filename),
    };

    file::save_file(&processed_text, &dest_path, config.force)?;

    Ok(())
}

fn download_all_licenses(config: &LicenseDownloadConfig) -> Result<()> {
    let fetcher = Fetcher::new();

    let pb = progress::spinner("Fetching SPDX license list...");
    let licenses_data = fetcher.fetch_json(SPDX_LICENSE_LIST_URL)?;
    pb.set_message("Parsing license list...");

    let licenses = licenses_data
        .get("licenses")
        .and_then(|l| l.as_array())
        .ok_or_else(|| anyhow!("Failed to parse SPDX licenses list"))?;

    pb.finish_and_clear();

    let active_licenses: Vec<_> = licenses
        .iter()
        .filter(|license| {
            !license
                .get("isDeprecatedLicenseId")
                .and_then(|d| d.as_bool())
                .unwrap_or(false)
        })
        .collect();

    println!(
        "Found {} active licenses. Downloading...",
        active_licenses.len()
    );

    for license in active_licenses {
        let license_id = license
            .get("licenseId")
            .and_then(|id| id.as_str())
            .ok_or_else(|| anyhow!("License ID not found"))?;

        if let Err(e) = download_single_license(license_id, config, Some(license_id.to_string())) {
            eprintln!(
                "{}",
                format!("⚠️  Failed to download {}: {}", license_id, e).red()
            );
        }
    }

    Ok(())
}

// ------------ HANDLE PLACEHOLDERS ------------

fn process_placeholders(
    license_text: &str,
    interactive: bool,
    placeholder_params: &HashMap<String, String>,
) -> Result<String> {
    let square_bracket_re = Regex::new(r"\[([^\]]+)\]")?;
    let angle_bracket_re = Regex::new(r"<([^>]+)>")?;

    // Collect all unique placeholders
    let mut placeholders = std::collections::HashSet::new();
    for caps in square_bracket_re.captures_iter(license_text) {
        if let Some(m) = caps.get(1) {
            placeholders.insert(m.as_str().to_string());
        }
    }
    for caps in angle_bracket_re.captures_iter(license_text) {
        if let Some(m) = caps.get(1) {
            placeholders.insert(m.as_str().to_string());
        }
    }

    if placeholders.is_empty() {
        println!("{}", "✓ No placeholders found in license text.".green());

        // Warn about unused parameters when no placeholders exist
        if !placeholder_params.is_empty() {
            println!(
                "{} {} parameter(s) provided but no placeholders found:",
                "⚠".yellow(),
                placeholder_params.len()
            );
            for (key, _) in placeholder_params {
                println!("  - {}", key);
            }
        }

        return Ok(license_text.to_string());
    } else if !interactive && placeholder_params.is_empty() {
        println!(
            "{} License contains placeholders. Use --interactive or --param PLACEHOLDER=VALUE to fill them.",
            "⚠".yellow()
        );
    }

    // Prepare normalized params for matching
    let normalized_params: HashMap<String, &String> = placeholder_params
        .iter()
        .map(|(k, v)| (normalize_placeholder_key(k), v))
        .collect();

    // Track which parameters are actually used
    let mut used_params = std::collections::HashSet::new();
    let mut unfilled_placeholders = Vec::new();

    let mut result = license_text.to_string();
    for ph in &placeholders {
        let norm_ph = normalize_placeholder_key(ph);

        let replacement = if let Some(val) = normalized_params.get(&norm_ph) {
            used_params.insert(norm_ph.clone());
            val.to_string()
        } else if interactive {
            let user_input = prompt_for_placeholder(ph);
            if user_input == format!("[{}]", ph) {
                unfilled_placeholders.push(ph.clone());
            }
            user_input
        } else {
            // Keep original placeholder and track as unfilled
            unfilled_placeholders.push(ph.clone());
            format!("[{}]", ph)
        };

        // Replace both [placeholder] and <placeholder>
        result = square_bracket_re
            .replace_all(&result, |caps: &regex::Captures| {
                if normalize_placeholder_key(&caps[1]) == norm_ph {
                    replacement.clone()
                } else {
                    caps[0].to_string()
                }
            })
            .to_string();
        result = angle_bracket_re
            .replace_all(&result, |caps: &regex::Captures| {
                if normalize_placeholder_key(&caps[1]) == norm_ph {
                    replacement.clone()
                } else {
                    caps[0].to_string()
                }
            })
            .to_string();
    }

    // Warning for unused parameters
    let unused_params: Vec<&String> = placeholder_params
        .keys()
        .filter(|k| !used_params.contains(&normalize_placeholder_key(k)))
        .collect();

    if !unused_params.is_empty() {
        println!(
            "{} Warning: {} unused parameter(s):",
            "⚠".yellow(),
            unused_params.len()
        );
        for param in unused_params {
            println!("  - {}", param);
        }
        println!("  Double-check parameter names match placeholders in the license.");
    }

    // Warning for unfilled placeholders
    if !unfilled_placeholders.is_empty() {
        println!(
            "{} Warning: {} placeholder(s) remain unfilled:",
            "⚠".yellow(),
            unfilled_placeholders.len()
        );
        for ph in &unfilled_placeholders {
            println!("  - [{}]", ph);
        }
        println!("  Use --interactive or --param to provide values for these placeholders.");
    }

    // Summary message for user verification
    let filled_count = placeholders.len() - unfilled_placeholders.len();
    if filled_count > 0 {
        println!(
            "{} Filled {} out of {} placeholder(s).",
            "✓".green(),
            filled_count,
            placeholders.len()
        );
        println!(
            "{} Please carefully review the license text above for any missed or incorrect placeholders.",
            "⚠".yellow()
        );
    }

    Ok(result)
}

fn normalize_placeholder_key(s: &str) -> String {
    s.trim().to_lowercase().replace(' ', "-")
}

fn prompt_for_placeholder(placeholder_content: &str) -> String {
    print!("Enter value for '{}': ", placeholder_content);
    let _ = io::stdout().flush();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let input = input.trim();
        if !input.is_empty() {
            input.to_string()
        } else {
            format!("[{}]", placeholder_content)
        }
    } else {
        format!("[{}]", placeholder_content)
    }
}
