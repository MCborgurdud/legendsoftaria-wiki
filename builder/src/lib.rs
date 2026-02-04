mod config;
mod data;
mod output;
mod postprocess;
mod render;
mod watch;
mod server;

use std::path::Path;

pub fn build_wiki(base_path: Option<&Path>) -> anyhow::Result<()> {
    if let Some(path) = base_path {
        config::set_base_path(path);
    }

    println!("Wiki Builder - Legends of Taria");
    println!("================================");

    output::copy_static_assets()?;
    output::copy_root_files()?;

    let tera = render::init_tera()?;

    let items = data::load_items()?;
    println!("Loaded {} items", items.len());

    let npcs = data::load_npcs()?;
    println!("Loaded {} npcs", npcs.len());

    let bosses = data::load_bosses()?;
    println!("Loaded {} bosses", bosses.len());

    let boss_rooms = data::load_boss_rooms()?;
    println!("Loaded {} boss rooms", boss_rooms.len());

    let pages = data::load_pages()?;
    println!("Loaded {} pages", pages.len());

    render::render_items(&tera, &items)?;
    render::render_npcs(&tera, &npcs, &items)?;
    render::render_regular_pages(&tera, &pages)?;
    render::render_bosses(&tera, &bosses, &items)?;
    render::render_boss_rooms(&tera, &boss_rooms, &items)?;
    render::render_indexes(&tera, &items, &npcs, &bosses, &boss_rooms, &pages)?;

    println!("================================");
    println!("Wiki generated successfully!");
    Ok(())
}

pub fn watch_mode(base_path: &Path) -> anyhow::Result<()> {
    watch::watch_mode(base_path)
}