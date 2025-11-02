use colored::*;
use std::collections::HashMap;

use crate::utils::cache::{Cache, CacheManager};

use super::ensure_gitignore_cache;

#[derive(clap::Args, Clone, Debug)]
pub struct ListArgs {
    /// Show popular templates
    #[arg(short = 'p', long)]
    pub popular: bool,

    /// Show global templates
    #[arg(short = 'g', long)]
    pub global: bool,

    /// Show community templates
    #[arg(short = 'c', long)]
    pub community: bool,

    /// Update the gitignore cache
    #[arg(long = "update-cache", default_value = "false")]
    pub update_cache: bool,
}

impl super::Runnable for ListArgs {
    fn run(&self) -> anyhow::Result<()> {
        let mut cache_manager = CacheManager::new()?;

        let cache: Cache<String> = ensure_gitignore_cache(&mut cache_manager, self.update_cache)?;

        // Filter templates based on arguments
        let templates = filter_templates(&cache, self);

        // Display results
        display_templates(templates);

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct GitIgnoreTemplate {
    pub name: String,
    pub path: String,
    pub category: String,
}

fn filter_templates(cache: &Cache<String>, args: &ListArgs) -> Vec<GitIgnoreTemplate> {
    let mut templates = Vec::new();

    // Determine which categories to include
    let show_popular = args.popular || (!args.popular && !args.global && !args.community);
    let show_global = args.global || (!args.popular && !args.global && !args.community);
    let show_community = args.community || (!args.popular && !args.global && !args.community);

    for (key, entry) in &cache.entries {
        let path = &entry.data;
        let category = determine_category(path);

        let should_include = match category.as_str() {
            "popular" => show_popular,
            "global" => show_global,
            "community" => show_community,
            _ => false,
        };

        if should_include {
            templates.push(GitIgnoreTemplate {
                name: key.clone(),
                path: path.clone(),
                category,
            });
        }
    }

    // Sort by name
    templates.sort_by(|a, b| a.name.cmp(&b.name));
    templates
}

fn determine_category(path: &str) -> String {
    if path.starts_with("Global/") {
        "global".to_string()
    } else if path.starts_with("community/") {
        "community".to_string()
    } else {
        "popular".to_string()
    }
}

fn display_templates(templates: Vec<GitIgnoreTemplate>) {
    if templates.is_empty() {
        println!("No gitignore templates found.");
        return;
    }

    println!("{}", "✓ Available gitignore templates:".green());

    // Group by category for better display
    let mut by_category: HashMap<String, Vec<&GitIgnoreTemplate>> = HashMap::new();
    for template in &templates {
        by_category
            .entry(template.category.clone())
            .or_default()
            .push(template);
    }

    for (category, mut templates) in by_category {
        templates.sort_by(|a, b| a.name.cmp(&b.name));

        println!("\n{}", category.to_uppercase().bold());
        for template in templates {
            println!(
                "  {} {:<20} ({})",
                ">".green(),
                template.name,
                template.path
            );
        }
    }
}
