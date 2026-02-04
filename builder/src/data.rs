use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::config;

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(alias = "type")]
    pub item_type: String,
    #[serde(default)]
    pub damage: Option<i32>,
    #[serde(default)]
    pub armor: Option<i32>,
    #[serde(default)]
    pub healing: Option<i32>,
    #[serde(default)]
    pub level_requirement: i32,
    #[serde(default)]
    pub acquisition: String,
    #[serde(default)]
    pub sell_price: Option<u64>,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Boss {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub location: String,
    pub role: String,
    pub description: String,
    #[serde(default)]
    pub level: Option<i32>,
    #[serde(default)]
    pub hitpoints: Option<i32>,
    #[serde(default)]
    pub drops: Vec<String>,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub defense: Option<i32>,
    #[serde(default)]
    pub magic: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Npc {
    pub id: String,
    pub name: String,
    pub location: String,
    pub role: String,
    pub description: String,
    #[serde(default)]
    pub level: Option<i32>,
    #[serde(default)]
    pub hitpoints: Option<i32>,
    #[serde(default)]
    pub drops: Vec<String>,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Serialize)]
pub struct Page {
    pub slug: String,
    pub title: String,
    pub body_html: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BossRoom {
    pub id: String,
    pub name: String,
    pub location: String,
    pub role: String,
    pub description: String,
    #[serde(default)]
    pub bosses: Vec<Boss>,
    #[serde(default)]
    pub level: Option<i32>,
    #[serde(default)]
    pub hitpoints: Option<i32>,
    #[serde(default)]
    pub notes: String,
}

/// Enriched drop information for rendering NPC drops with item links and prices
#[derive(Debug, Deserialize, Serialize)]
pub struct EnrichedDrop {
    pub item_id: String,
    pub item_name: String,
    pub item_type: String,
    pub sell_price: Option<u64>,
    pub link_html: String,
}

pub fn load_npcs() -> Result<Vec<Npc>> {
    let mut npcs = Vec::new();
    let dir = config::data_dir().join("npcs");

    if !dir.exists() {
        return Ok(npcs);
    }

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let mut file =
            fs::File::open(&path).with_context(|| format!("failed to open npc file {:?}", path))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let npc: Npc = serde_json::from_str(&buf)
            .with_context(|| format!("failed to parse npc JSON {:?}", path))?;
        npcs.push(npc);
    }

    npcs.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(npcs)
}

pub fn load_pages() -> Result<Vec<Page>> {
    let mut pages = Vec::new();
    let dir = config::data_dir().join("pages");

    if !dir.exists() {
        return Ok(pages);
    }

    for entry in WalkDir::new(&dir) {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let mut file = fs::File::open(&path)
            .with_context(|| format!("failed to open page file {:?}", path))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let parser = Parser::new_ext(&buf, Options::all());
        let mut body_html = String::new();
        html::push_html(&mut body_html, parser);

        let slug = path
            .strip_prefix(&dir)
            .unwrap()
            .with_extension("")
            .to_string_lossy()
            .to_string();

        let title = slug
            .rsplit('/')
            .next()
            .unwrap_or(&slug)
            .replace('-', " ")
            .to_case(Case::Title);

        pages.push(Page {
            slug,
            title,
            body_html,
        });
    }

    Ok(pages)
}


pub fn load_items() -> Result<Vec<Item>> {
    let mut items = Vec::new();
    let dir = config::data_dir().join("items");

    if !dir.exists() {
        return Ok(items);
    }

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let mut file =
            fs::File::open(&path).with_context(|| format!("failed to open item file {:?}", path))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let item: Item = serde_json::from_str(&buf)
            .with_context(|| format!("failed to parse item JSON {:?}", path))?;
        items.push(item);
    }

    items.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(items)
}

pub fn load_bosses() -> Result<Vec<Boss>> {
    let mut bosses = Vec::new();
    let dir = config::data_dir().join("bosses");

    if !dir.exists() {
        return Ok(bosses);
    }

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let mut file =
            fs::File::open(&path).with_context(|| format!("failed to open boss file {:?}", path))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let boss: Boss = serde_json::from_str(&buf)
            .with_context(|| format!("failed to parse boss JSON {:?}", path))?;
        bosses.push(boss);
    }

    bosses.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(bosses)
}

pub fn load_boss_rooms() -> Result<Vec<BossRoom>> {
    let mut boss_rooms = Vec::new();
    let dir = config::data_dir().join("boss-rooms");

    if !dir.exists() {
        return Ok(boss_rooms);
    }

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }       
    }
}
