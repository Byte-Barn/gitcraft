use clap::Subcommand;

use crate::commands::base::Runnable;
use crate::utils::cache::{Cache, CacheManager};
use crate::utils::progress;
use crate::utils::remote::Fetcher;

pub mod add;
pub mod list;
pub mod preview;

// Global constants - these can stay in the main module file
const CACHE_MAX_AGE_SECONDS: u64 = 60 * 60 * 24 * 30; // 30 days
const SPDX_LICENSE_DETAILS_BASE_URL: &str =
    "https://raw.githubusercontent.com/spdx/license-list-data/main/json/details";
const SPDX_LICENSE_LIST_URL: &str =
    "https://raw.githubusercontent.com/spdx/license-list-data/main/json/licenses.json";
const CHOOSEALICENSE_RAW_BASE_URL: &str =
    "https://raw.githubusercontent.com/github/choosealicense.com/gh-pages/_licenses";

const GITHUB_LICENSES_CACHE_NAME: &str = "github_licenses_cache";
const GITHUB_LICENSE_API_URL: &str = "https://api.github.com/licenses";

const SPDX_CACHE_NAME: &str = "spdx_license_cache";

#[derive(Subcommand)]
pub enum Command {
    /// Add one or more licenses to the repository
    Add(add::AddArgs),
    /// List available licenses
    List(list::ListArgs),
    /// Preview a specific license
    Preview(preview::PreviewArgs),
}

impl Command {
    pub fn execute(&self) -> anyhow::Result<()> {
        match self {
            Command::Add(args) => args.run(),
            Command::List(args) => args.run(),
            Command::Preview(args) => args.run(),
        }
    }
}

fn ensure_spdx_license_cache(
    cache_manager: &mut CacheManager,
    update_cache: bool,
) -> Result<Cache<serde_json::Value>, anyhow::Error> {
    // Only print if we are updating the cache
    let should_update = cache_manager
        .should_update_cache::<serde_json::Value>(SPDX_CACHE_NAME, CACHE_MAX_AGE_SECONDS)?;

    if !should_update && !update_cache {
        let cache = cache_manager.load_cache(SPDX_CACHE_NAME)?;
        // Only print if running in verbose/debug mode (not implemented here)
        // e.g., println!("Loaded license template cache ({} templates)", cache.entries.len());
        return Ok(cache);
    }

    let pb = progress::spinner("Updating license template cache...");

    let fetcher = Fetcher::new();
    let url = SPDX_LICENSE_LIST_URL;

    let data = fetcher.fetch_json(url)?;
    let mut cache = Cache::new();

    if let Some(licenses) = data.get("licenses").and_then(|v| v.as_array()) {
        for entry in licenses {
            if let Some(license_id) = entry.get("licenseId").and_then(|id| id.as_str()) {
                // Use the licenseId as the cache key, and the whole license entry as the value (as JSON string)
                cache.insert(license_id.to_string(), entry.clone());
            }
        }
    }

    pb.finish_and_clear();
    println!(
        "License template cache updated ({} templates available).",
        cache.entries.len()
    );

    cache_manager.save_cache(SPDX_CACHE_NAME, &cache)?;
    Ok(cache)
}

fn ensure_github_api_license_cache(
    cache_manager: &mut CacheManager,
    update_cache: bool,
) -> Result<Cache<serde_json::Value>, anyhow::Error> {
    let should_update = cache_manager.should_update_cache::<serde_json::Value>(
        GITHUB_LICENSES_CACHE_NAME,
        CACHE_MAX_AGE_SECONDS,
    )?;

    if !should_update && !update_cache {
        let cache = cache_manager.load_cache(GITHUB_LICENSES_CACHE_NAME)?;
        // Only print if running in verbose/debug mode (not implemented here)
        // e.g., println!("Loaded GitHub licenses cache ({} licenses)", cache.entries.len());
        return Ok(cache);
    }

    let pb = progress::spinner("Updating popular licenses cache...");
    let fetcher = Fetcher::new();
    let url = GITHUB_LICENSE_API_URL;
    let content = fetcher.fetch_json(url)?;
    let mut new_cache = Cache::new();

    if let Some(array) = content.as_array() {
        for license in array {
            if let Some(id) = license.get("key").and_then(|k| k.as_str()) {
                new_cache.insert(id.to_string(), license.clone());
            }
        }
    }

    pb.finish_and_clear();
    println!(
        "Popular licenses cache updated ({} licenses available).",
        new_cache.entries.len()
    );

    cache_manager.save_cache(GITHUB_LICENSES_CACHE_NAME, &new_cache)?;
    Ok(new_cache)
}
