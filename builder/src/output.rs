use std::fs;

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::config;

/// Copy static assets (CSS, images) from html/assets to out/assets
pub fn copy_static_assets() -> Result<()> {
    let src_assets = config::html_dir().join("assets");
    let dest_assets = config::output_dir().join("assets");

    if !src_assets.exists() {
        println!("No assets directory found, skipping asset copy");
        return Ok(());
    }

    let mut count = 0;

    for entry in WalkDir::new(&src_assets) {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let rel = path.strip_prefix(&src_assets).unwrap();
        let dest_path = dest_assets.join(rel);

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(path, &dest_path).with_context(|| format!("failed to copy {:?}", path))?;
        count += 1;
    }

    println!("Copied {} static assets", count);
    Ok(())
}
