use colored::*;

use crate::utils::cache::{Cache, CacheManager};
use crate::utils::pattern::filter_by_wildcard;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

// SPDX license list URL
use super::{SPDX_LICENSE_LIST_URL, ensure_github_api_license_cache, ensure_spdx_license_cache};

#[derive(clap::Args, Clone, Debug)]
pub struct ListArgs {
    /// Show only popular/common licenses
    #[arg(long, short)]
    pub popular: bool,

    /// non-software licenses
    #[arg(long, short)]
    pub non_software: bool,

    /// Search for licenses containing this term
    #[arg(long, short)]
    pub search: Option<String>,

    /// Show deprecated licenses as well
    #[arg(long)]
    pub include_deprecated: bool,

    /// Update the license cache before listing
    #[arg(long)]
    pub update_cache: bool,

    #[arg(allow_hyphen_values = true)]
    pub args: Vec<String>,

    /// Show only OSI-approved licenses
    #[arg(long)]
    pub osi_approved: bool,

    /// Show only FSF libre-approved licenses
    #[arg(long)]
    pub fsf_libre: bool,
}

impl super::Runnable for ListArgs {
    fn run(&self) -> anyhow::Result<()> {
        // Handle any unknown arguments
        if !self.args.is_empty() {
            for arg in &self.args {
                return Err(anyhow::anyhow!("Unknown argument: {}", arg));
            }
        }

        // License Args
        let license_args = LicenseArgs {
            update_cache: self.update_cache,
            search: self.search.clone(),
            include_deprecated: self.include_deprecated,
            osi_approved: self.osi_approved,
            fsf_libre: self.fsf_libre,
        };

        if self.popular {
            return list_popular_licenses(license_args);
        } else if self.non_software {
            return list_non_software_licenses(self.update_cache);
        } else {
            list_all_licenses(license_args)
                .map_err(|e| anyhow::anyhow!("Failed to list licenses: {}", e))
        }
    }
}

struct LicenseArgs {
    update_cache: bool,
    search: Option<String>,
    include_deprecated: bool,
    osi_approved: bool,
    fsf_libre: bool,
}

fn list_popular_licenses(args: LicenseArgs) -> anyhow::Result<()> {
    let mut cache_manager = CacheManager::new()?;

    let cache: Cache<serde_json::Value> =
        ensure_github_api_license_cache(&mut cache_manager, args.update_cache)?;

    // If search parameter is passed, filter licenses by closest matches
    if let Some(search) = &args.search {
        let mut matches = Vec::new();

        // Collect all IDs and names for wildcard filtering
        let mut all_items = Vec::new();
        for (id, entry) in &cache.entries {
            all_items.push(id.clone());
            if let Some(name) = entry.data.get("name").and_then(|n| n.as_str()) {
                all_items.push(name.to_string());
            }
        }

        // Use wildcard pattern matching
        let filtered = filter_by_wildcard(search, &all_items);

        for (id, entry) in &cache.entries {
            let name = entry
                .data
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("");
            if filtered.contains(id) || filtered.contains(&name.to_string()) {
                matches.push((id, entry));
            }
        }

        if matches.is_empty() {
            println!("No popular licenses found matching '{}'", search);
            return Ok(());
        }

        println!(
            "{} Popular licenses matching '{}' ({} found):",
            "✓".green(),
            search,
            matches.len()
        );
        println!();

        for (id, entry) in matches {
            if let Some(name) = entry.data.get("name").and_then(|n| n.as_str()) {
                println!("  {} {:<20} {}", ">".green(), id, name);
            } else {
                println!("{}", id);
            }
        }

        return Ok(());
    }

    for (id, entry) in &cache.entries {
        if let Some(name) = entry.data.get("name").and_then(|n| n.as_str()) {
            println!("  {} {:<20} {}", ">".green(), id, name);
        } else {
            println!("  {} {:<20} {:?}", ">".green(), id, entry.data);
            println!("  {} {}", ">".green(), id);
        }
    }

    Ok(())
}

fn list_all_licenses(args: LicenseArgs) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    let pb = progress::spinner("Fetching SPDX license list...");

    let licenses_data = fetcher.fetch_json(SPDX_LICENSE_LIST_URL)?;

    pb.set_message("Parsing license list...");

    let licenses = licenses_data
        .get("licenses")
        .and_then(|l| l.as_array())
        .ok_or_else(|| anyhow::anyhow!("Failed to parse SPDX licenses list"))?;

    pb.set_message("Successfully fetched licenses");
    pb.finish_and_clear();

    // Filter and collect licenses
    let mut filtered_licenses = Vec::new();

    for license in licenses {
        let license_id = license
            .get("licenseId")
            .and_then(|id| id.as_str())
            .unwrap_or("unknown");

        let license_name = license
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("Unknown License");

        let is_deprecated = license
            .get("isDeprecatedLicenseId")
            .and_then(|d| d.as_bool())
            .unwrap_or(false);

        // Skip deprecated licenses unless explicitly requested
        if is_deprecated && !args.include_deprecated {
            continue;
        }

        // Apply search filter if provided, supporting wildcard patterns (case-insensitive)
        if let Some(search) = &args.search {
            let mut search_lower = search.to_lowercase();
            // If the search does not end with '*' or '?', add a trailing '*'
            if !search_lower.ends_with('*') && !search_lower.ends_with('?') {
                search_lower.push('*');
            }
            let candidates = vec![license_id.to_lowercase(), license_name.to_lowercase()];
            let filtered = filter_by_wildcard(&search_lower, &candidates);

            if !filtered.contains(&license_id.to_lowercase())
                && !filtered.contains(&license_name.to_lowercase())
            {
                continue;
            }
        }

        // Filter by OSI-approved if requested
        if args.osi_approved {
            let osi_approved = license
                .get("isOsiApproved")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if !osi_approved {
                continue;
            }
        }

        // Filter by FSF libre if requested
        if args.fsf_libre {
            let fsf_libre = license
                .get("isFsfLibre")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if !fsf_libre {
                continue;
            }
        }

        filtered_licenses.push((license_id, license_name, is_deprecated, license));
    }

    // Sort by license ID for consistent output
    filtered_licenses.sort_by(|a, b| a.0.cmp(b.0));

    // Display results
    if filtered_licenses.is_empty() {
        if let Some(search) = &args.search {
            println!("No licenses found matching '{}'", search);
        } else {
            println!("No licenses found");
        }
        return Ok(());
    }

    let header = if let Some(search) = &args.search {
        format!("Licenses matching '{}'", search)
    } else {
        "Available SPDX licenses".to_string()
    };

    println!(
        "{} {} ({} found):",
        "✓".green(),
        header,
        filtered_licenses.len()
    );
    println!();

    display_simple_licenses(&filtered_licenses);

    if !args.include_deprecated {
        println!("\nNote:  deprecated licenses are hidden. Use --include-deprecated to show them.");
    }

    Ok(())
}

fn display_simple_licenses(licenses: &[(&str, &str, bool, &serde_json::Value)]) {
    for (id, name, is_deprecated, _) in licenses {
        let deprecated_marker = if *is_deprecated { " (deprecated)" } else { "" };
        println!(
            "  {} {:<20} {}{}",
            ">".green(),
            id,
            name,
            deprecated_marker.yellow()
        );
    }
}

/// Source: https://choosealicense.com/non-software/
fn list_non_software_licenses(update_cache: bool) -> anyhow::Result<()> {
    let mut cache_manager = CacheManager::new()?;

    // License IDs relevant for non-software works
    // let non_software_license_ids = [
    //     "CC0-1.0",
    //     "CC-BY-4.0",
    //     "CC-BY-SA-4.0",
    //     "OFL-1.1",
    //     "CERN-OHL-P-2.0",
    //     "CERN-OHL-W-2.0",
    //     "CERN-OHL-S-2.0",
    // ];

    // Use SPDX cache for license info
    let cache: Cache<serde_json::Value> =
        ensure_spdx_license_cache(&mut cache_manager, update_cache)?;

    println!("{}", "✓ Non-Software Licenses:".green());
    println!();

    // Data, media, etc.
    println!("{}", "Data, media, etc.".bold());
    for id in &["CC0-1.0", "CC-BY-4.0", "CC-BY-SA-4.0"] {
        if let Some(entry) = cache.entries.get(*id) {
            let name = entry
                .data
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or(id);
            println!("  {} {:<20} {}", ">".green(), id, name);
        } else {
            println!(
                "  {} {:<20} {}",
                "!".red(),
                id,
                "(not found in SPDX cache)".yellow()
            );
        }
    }
    println!(
        "    Creative Commons licenses are for non-software material. Not recommended for software/hardware.\n"
    );

    // Fonts
    println!("{}", "Fonts".bold());
    if let Some(entry) = cache.entries.get("OFL-1.1") {
        let name = entry
            .data
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("OFL-1.1");
        println!("  {} {:<20} {}\n", ">".green(), "OFL-1.1", name);
    } else {
        println!(
            "  {} {:<20} {}",
            "!".red(),
            "OFL-1.1",
            "(not found in SPDX cache)".yellow()
        );
    }

    // Hardware
    println!("{}", "Hardware".bold());
    for id in &["CERN-OHL-P-2.0", "CERN-OHL-W-2.0", "CERN-OHL-S-2.0"] {
        if let Some(entry) = cache.entries.get(*id) {
            let name = entry
                .data
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or(id);
            println!("  {} {:<20} {}", ">".green(), id, name);
        } else {
            println!(
                "  {} {:<20} {}",
                "!".red(),
                id,
                "(not found in SPDX cache)".yellow()
            );
        }
    }
    println!();

    println!("For more information, see: https://choosealicense.com/non-software/");

    Ok(())
}
