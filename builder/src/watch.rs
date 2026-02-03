use anyhow::Result;
use notify::Watcher;
use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::config;

/// Watch mode: monitors source files and rebuilds on changes
pub fn watch_mode(base_path: &Path) -> Result<()> {
    println!();
    println!("╔════════════════════════════════════════╗");
    println!("║   🔍 Wiki Builder - WATCH MODE 👀    ║");
    println!("║   Listening for file changes...        ║");
    println!("╠════════════════════════════════════════╣");
    println!("║  Press Ctrl+C to stop                  ║");
    println!("║  Hint: Run with '--build' for one-time ║");
    println!("╚════════════════════════════════════════╝");
    println!();

    let (tx, rx) = mpsc::channel();

    // Create a debounced watcher - waits for file changes to settle
    let mut watcher = notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
        if let Ok(event) = event {
            // Filter out non-relevant events and emit on file operations
            match event.kind {
                notify::EventKind::Create(_)
                | notify::EventKind::Modify(_)
                | notify::EventKind::Remove(_) => {
                    // Only watch relevant directories
                    if event.paths.iter().any(|p| {
                        let path_str = p.to_string_lossy().to_lowercase();
                        path_str.contains("site/data")
                            || path_str.contains("site/html")
                            || path_str.contains("site/templates")
                    }) {
                        let _ = tx.send(());
                    }
                }
                _ => {}
            }
        }
    })?;

    // Watch the site directory for changes
    let site_dir = base_path.join("site");
    if site_dir.exists() {
        watcher.watch(&site_dir, notify::RecursiveMode::Recursive)?;
    }

    println!("✓ Watching for changes in site/\n");

    let mut last_build = Instant::now();
    let debounce_duration = Duration::from_millis(300);

    loop {
        // Wait for file change events with debouncing
        if let Ok(()) = rx.recv() {
            // Debounce: ignore rapid successive changes
            if last_build.elapsed() < debounce_duration {
                std::thread::sleep(debounce_duration);
                // Drain remaining events in the queue
                while rx.try_recv().is_ok() {
                    std::thread::sleep(Duration::from_millis(50));
                }
            }

            last_build = Instant::now();
            println!("┌─ 📝 Changes detected...");

            // Run the build
            match run_build(base_path) {
                Ok(()) => {
                    println!("└─ ✓ Build successful!\n");
                }
                Err(e) => {
                    println!("└─ ✗ Build failed: {}\n", e);
                    print_error_box();
                    // Continue watching instead of crashing
                }
            }
        }
    }
}

fn run_build(base_path: &Path) -> Result<()> {
    config::set_base_path(base_path);

    crate::output::copy_static_assets()?;
    crate::output::copy_root_files()?;

    let tera = crate::render::init_tera()?;

    let items = crate::data::load_items()?;
    let npcs = crate::data::load_npcs()?;
    let pages = crate::data::load_pages()?;

    crate::render::render_items(&tera, &items)?;
    crate::render::render_npcs(&tera, &npcs, &items)?;
    crate::render::render_regular_pages(&tera, &pages)?;
    crate::render::render_indexes(&tera, &items, &npcs, &pages)?;

    Ok(())
}

fn print_error_box() {
    println!("┌──────────────────────────────────────┐");
    println!("│  ⚠ The build has errors above       │");
    println!("│  ⚠ Previous build remains intact     │");
    println!("│  👉 Fix the issues and save files    │");
    println!("└──────────────────────────────────────┘");
    println!();
}
