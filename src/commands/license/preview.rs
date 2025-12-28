use colored::*;

use super::{
    ensure_spdx_license_cache, CHOOSEALICENSE_RAW_BASE_URL, SPDX_LICENSE_DETAILS_BASE_URL,
    SPDX_LICENSE_LIST_URL,
};

use crate::utils::cache::{Cache, CacheManager};
use crate::utils::remote::Fetcher;
use serde::{Deserialize, Serialize};

#[derive(clap::Args, Debug, Clone)]
pub struct PreviewArgs {
    /// License ID (e.g. mit, apache-2.0)
    #[arg(value_name = "LICENSE")]
    pub id: String,

    /// Show description
    #[arg(long, short = 'd')]
    pub description: bool,

    /// Show permissions
    #[arg(long, short = 'p')]
    pub permissions: bool,

    /// Show limitations
    #[arg(long, short = 'l')]
    pub limitations: bool,

    /// Show conditions
    #[arg(long, short = 'c')]
    pub conditions: bool,

    /// Show all details
    #[arg(long, short = 'D')]
    pub details: bool,

    /// Update the license cache
    #[arg(long, short = 'u')]
    pub update_cache: bool,
}

impl super::Runnable for PreviewArgs {
    fn run(&self) -> anyhow::Result<()> {
        let mut cache_manager = CacheManager::new()?;

        let cache: Cache<serde_json::Value> =
            ensure_spdx_license_cache(&mut cache_manager, self.update_cache)?;

        let normalized_id = normalize_license_id(&self.id);
        let id_lower = normalized_id.to_lowercase();

        // Get the license name from the cache using normalized lowercase ID
        // Find the full license JSON entry in the cache by normalized ID
        let (license_key, license_json) = cache
            .entries
            .iter()
            .find_map(|(key, entry)| {
                if key.to_lowercase() == id_lower {
                    Some((key.clone(), entry.data.clone()))
                } else {
                    None
                }
            })
            .unwrap_or((normalized_id.clone(), serde_json::Value::Null));

        println!(
            "{} {} ({})\n",
            "License:".cyan(),
            license_json
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or(&normalized_id),
            self.id.to_uppercase()
        );

        // Try to fetch ChooseALicense metadata if available
        let choosealicense_meta = fetch_choosealicense_meta(&normalized_id);

        // If no flags, show full license
        if !self.description
            && !self.permissions
            && !self.limitations
            && !self.conditions
            && !self.details
        {
            let license_url = format!("{}/{}.json", SPDX_LICENSE_DETAILS_BASE_URL, license_key);
            show_full_license(&license_url)?;
            return Ok(());
        }

        // Fetch SPDX metadata if needed
        let license_metadata = if self.description || self.details {
            Some(get_license_metadata(&self.id)?)
        } else {
            None
        };

        // Show description
        if self.description || self.details {
            show_description(license_metadata.as_ref(), choosealicense_meta.as_ref())?;
        }
        // Show permissions
        if self.permissions || self.details {
            show_permissions(license_metadata.as_ref(), choosealicense_meta.as_ref())?;
        }
        // Show limitations
        if self.limitations || self.details {
            show_limitations(license_metadata.as_ref(), choosealicense_meta.as_ref())?;
        }
        // Show conditions
        if self.conditions || self.details {
            show_conditions(license_metadata.as_ref(), choosealicense_meta.as_ref())?;
        }

        // Show additional SPDX metadata
        if self.details {
            if let Some(metadata) = &license_metadata {
                show_spdx_metadata(metadata)?;
            }
        }
        // Show license text unless metadata-only is specified
        // if !self.metadata_only {
        //     let license_url = format!("{}/{}.json", SPDX_LICENSE_DETAILS_BASE_URL, license_key);
        //     show_full_license(&license_url)?;
        // }

        Ok(())
    }
}

fn fetch_choosealicense_meta(normalized_id: &str) -> Option<ChooseALicenseFile> {
    let url = format!("{}/{}.txt", CHOOSEALICENSE_RAW_BASE_URL, normalized_id);
    let fetcher = Fetcher::new();
    match fetcher.fetch_content(&url) {
        Ok(content) => parse_choosealicense_txt(&content).ok(),
        Err(_) => None,
    }
}

fn get_license_metadata(license_id: &str) -> anyhow::Result<serde_json::Value> {
    let fetcher = Fetcher::new();
    let licenses_data = fetcher.fetch_json(SPDX_LICENSE_LIST_URL)?;

    let licenses = licenses_data
        .get("licenses")
        .and_then(|l| l.as_array())
        .ok_or_else(|| anyhow::anyhow!("Failed to parse SPDX licenses list"))?;

    let license_id_lower = license_id.to_lowercase();
    for license in licenses {
        if let Some(id) = license.get("licenseId").and_then(|id| id.as_str()) {
            if id.to_lowercase() == license_id_lower {
                return Ok(license.clone());
            }
        }
    }

    Err(anyhow::anyhow!(
        "License '{}' not found in SPDX database",
        license_id
    ))
}

fn show_description(
    license_metadata: Option<&serde_json::Value>,
    choosealicense_meta: Option<&ChooseALicenseFile>,
) -> anyhow::Result<()> {
    // Show the license description from ChooseALicense if available,
    // otherwise fallback to SPDX metadata if present.
    println!("{}", "Description:".cyan());

    // Try ChooseALicense description first
    if let Some(meta) = choosealicense_meta {
        if let Some(desc) = &meta.meta.description {
            println!("{}", desc);
        } else {
            println!("  No description available from ChooseALicense.");
        }
    }
    // If no ChooseALicense description, try SPDX metadata
    else if let Some(metadata) = license_metadata {
        if let Some(desc) = metadata.get("detailsUrl").and_then(|d| d.as_str()) {
            println!("  See SPDX details: {}", desc);
        } else if let Some(desc) = metadata.get("name").and_then(|n| n.as_str()) {
            println!("  SPDX License Name: {}", desc);
        } else {
            println!("  No description available from SPDX.");
        }
    }
    // If neither source has a description
    else {
        println!("  No description available.");
    }

    println!();
    Ok(())
}

fn show_permissions(
    license_metadata: Option<&serde_json::Value>,
    choosealicense: Option<&ChooseALicenseFile>,
) -> anyhow::Result<()> {
    println!("{}", "Permissions:".green());
    if let Some(meta) = choosealicense {
        if let Some(perms) = &meta.meta.permissions {
            for perm in perms {
                println!("  ✓ {}", perm);
            }
        } else {
            println!("  Not available from ChooseALicense.");
        }
    } else if let Some(metadata) = license_metadata {
        if let Some(perms) = metadata.get("permissions").and_then(|p| p.as_array()) {
            for perm in perms {
                if let Some(p) = perm.as_str() {
                    println!("  ✓ {}", p);
                }
            }
        } else {
            println!("  Not available from SPDX.");
            if let Some(desc) = metadata.get("detailsUrl").and_then(|d| d.as_str()) {
                println!("  See SPDX details: {}", desc);
            }
        }
    } else {
        println!("  Not available.");
    }
    println!();
    Ok(())
}

fn show_limitations(
    license_metadata: Option<&serde_json::Value>,
    choosealicense: Option<&ChooseALicenseFile>,
) -> anyhow::Result<()> {
    println!("{}", "Limitations:".red());
    if let Some(meta) = choosealicense {
        if let Some(lims) = &meta.meta.limitations {
            for lim in lims {
                println!("  ✗ {}", lim);
            }
        } else {
            println!("  Not available from ChooseALicense.");
        }
    } else if let Some(metadata) = license_metadata {
        if let Some(lims) = metadata.get("limitations").and_then(|l| l.as_array()) {
            for lim in lims {
                if let Some(l) = lim.as_str() {
                    println!("  ✗ {}", l);
                }
            }
        } else {
            println!("  Not available from SPDX.");
            if let Some(desc) = metadata.get("detailsUrl").and_then(|d| d.as_str()) {
                println!("  See SPDX details: {}", desc);
            }
        }
    } else {
        println!("  Not available.");
    }
    println!();
    Ok(())
}

fn show_conditions(
    license_metadata: Option<&serde_json::Value>,
    choosealicense: Option<&ChooseALicenseFile>,
) -> anyhow::Result<()> {
    println!("{}", "Conditions:".yellow());
    if let Some(meta) = choosealicense {
        if let Some(conds) = &meta.meta.conditions {
            for cond in conds {
                println!("  ! {}", cond);
            }
        } else {
            println!("  Not available from ChooseALicense.");
        }
    } else if let Some(metadata) = license_metadata {
        if let Some(conds) = metadata.get("conditions").and_then(|c| c.as_array()) {
            for cond in conds {
                if let Some(c) = cond.as_str() {
                    println!("  ! {}", c);
                }
            }
        } else {
            println!("  Not available from SPDX.");
            if let Some(desc) = metadata.get("detailsUrl").and_then(|d| d.as_str()) {
                println!("  See SPDX details: {}", desc);
            }
        }
    } else {
        println!("  Not available.");
    }
    println!();
    Ok(())
}

fn show_spdx_metadata(license_metadata: &serde_json::Value) -> anyhow::Result<()> {
    println!("{}", "SPDX Metadata:".cyan());

    if let Some(license_id) = license_metadata.get("licenseId").and_then(|id| id.as_str()) {
        println!("  License ID: {}", license_id);
    }

    if let Some(osi_approved) = license_metadata
        .get("isOsiApproved")
        .and_then(|o| o.as_bool())
    {
        println!(
            "  OSI Approved: {}",
            if osi_approved { "✓ Yes" } else { "✗ No" }
        );
    }

    if let Some(fsf_libre) = license_metadata.get("isFsfLibre").and_then(|f| f.as_bool()) {
        println!("  FSF Libre: {}", if fsf_libre { "✓ Yes" } else { "✗ No" });
    }

    if let Some(deprecated) = license_metadata
        .get("isDeprecatedLicenseId")
        .and_then(|d| d.as_bool())
    {
        if deprecated {
            println!("  {}", "Status: DEPRECATED".yellow());
            if let Some(details_url) = license_metadata.get("detailsUrl").and_then(|d| d.as_str()) {
                println!("      {}", format!("See details: {}", details_url).yellow());
            }
        }
    }

    println!();
    Ok(())
}
fn show_full_license(url: &str) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    println!("{}", "License Text:".cyan());
    println!("{}", "─".repeat(80));

    match fetcher.fetch_json(url) {
        Ok(license_json) => {
            if let Some(license_text) = license_json.get("licenseText").and_then(|t| t.as_str()) {
                println!("{}", license_text);
            } else {
                println!("⚠️  License text not found.");
            }
        }
        Err(e) => {
            println!("⚠️  Could not fetch license text: {}", e);
            println!(
                "This may not be a valid SPDX license ID or the license text may not be available."
            );
            println!("Try using --update-cache to refresh the license database.");
        }
    }

    println!("{}", "─".repeat(80));
    Ok(())
}

// Helper to normalize license ID (e.g. "mit" -> "MIT")
fn normalize_license_id(id: &str) -> String {
    id.trim().to_lowercase()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseALicenseMeta {
    pub title: String,
    #[serde(rename = "spdx-id")]
    pub spdx_id: String,
    pub featured: Option<bool>,
    pub hidden: Option<bool>,
    pub description: Option<String>,
    pub how: Option<String>,
    pub using: Option<std::collections::HashMap<String, String>>,
    pub permissions: Option<Vec<String>>,
    pub conditions: Option<Vec<String>>,
    pub limitations: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct ChooseALicenseFile {
    pub meta: ChooseALicenseMeta,
    // pub body: String,   // License body text
}

/// Parses a choosealicense.com license text file into structured data.
/// Returns ChooseALicenseFile with parsed metadata and license body.
pub fn parse_choosealicense_txt(content: &str) -> anyhow::Result<ChooseALicenseFile> {
    // Find the positions of the YAML front matter and the body
    let parts = content.splitn(3, "---").collect::<Vec<_>>();
    if parts.len() < 3 {
        return Err(anyhow::anyhow!("Invalid choosealicense file format"));
    }
    // parts[1] is the YAML metadata, parts[2] is the license body
    let meta_str = parts[1].trim();
    // let body = parts[2].trim().to_string(); // License body text (not used)

    let meta: ChooseALicenseMeta = serde_yaml::from_str(meta_str)?;
    Ok(ChooseALicenseFile { meta })
}
