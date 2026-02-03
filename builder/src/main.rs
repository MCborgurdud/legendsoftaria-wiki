use anyhow::Result;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut args = env::args();
    let _exe_path = args.next().unwrap_or_default();

    // Find workspace root - go up from current directory until we find 'site' and 'builder'
    let mut base_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    
    // If we're in the builder directory, go up one level
    if base_path.file_name().map_or(false, |n| n == "builder") {
        base_path.pop();
    }

    let mode = args.next();

    match mode.as_deref() {
        Some("--watch") | Some("-w") => {
            wiki_builder::watch_mode(&base_path)?;
        }
        Some("--build") | Some("-b") | None => {
            wiki_builder::build_wiki(Some(&base_path))?;
        }
        Some("--help") | Some("-h") => {
            print_help();
        }
        Some(arg) => {
            eprintln!("Unknown argument: {}", arg);
            print_help();
            std::process::exit(1);
        }
    }

    Ok(())
}

fn print_help() {
    println!("Wiki Builder - Legends of Taria");
    println!();
    println!("USAGE:");
    println!("  wiki-builder [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("  --build, -b    Build once and exit (default)");
    println!("  --watch, -w    Watch for changes and rebuild automatically");
    println!("  --help, -h     Show this help message");
}

