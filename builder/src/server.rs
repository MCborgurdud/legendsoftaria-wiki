use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::thread;

use anyhow::{Context, Result};
use tiny_http::{Header, Response, Server, StatusCode};

use crate::config;

pub fn start_server(base_path: &Path, build_counter: Arc<AtomicU64>) -> Result<()> {
    config::set_base_path(base_path);

    let addr = "127.0.0.1:8080";
    let server = Server::http(addr)
        .map_err(|e| anyhow::anyhow!("failed to start web server: {}", e))?;

    println!("✓ Server running at http://{}/", addr);

    let out_dir = config::output_dir();

    thread::spawn(move || {
        for request in server.incoming_requests() {
            if let Err(err) = handle_request(request, &out_dir, &build_counter) {
                eprintln!("[server] error: {}", err);
            }
        }
    });

    Ok(())
}

fn handle_request(
    request: tiny_http::Request,
    out_dir: &Path,
    build_counter: &Arc<AtomicU64>,
) -> Result<()> {
    let url = request.url();

    if url == "/__reload" {
        let build_id = build_counter.load(Ordering::Relaxed);
        let body = format!("{{\"build\":{}}}", build_id);
        let response = Response::from_string(body)
            .with_status_code(StatusCode(200))
            .with_header(header("Content-Type", "application/json"));
        request.respond(response)?;
        return Ok(());
    }

    let path = sanitize_url_path(url)?;
    let mut full_path = out_dir.join(&path);

    if full_path.is_dir() {
        full_path = full_path.join("index.html");
    }

    if !full_path.exists() {
        let response = Response::from_string("Not Found")
            .with_status_code(StatusCode(404))
            .with_header(header("Content-Type", "text/plain; charset=utf-8"));
        request.respond(response)?;
        return Ok(());
    }

    let data = fs::read(&full_path)
        .with_context(|| format!("failed to read file {:?}", full_path))?;

    let content_type = content_type_for_path(&full_path);
    let response = Response::from_data(data)
        .with_status_code(StatusCode(200))
        .with_header(header("Content-Type", content_type));

    request.respond(response)?;
    Ok(())
}

fn sanitize_url_path(url: &str) -> Result<PathBuf> {
    let mut path = url.trim_start_matches('/').to_string();

    if path.is_empty() {
        path = "index.html".to_string();
    }

    if path.contains("..") {
        anyhow::bail!("invalid path");
    }

    // Basic percent-decoding for spaces
    let path = path.replace("%20", " ");
    Ok(PathBuf::from(path))
}

fn content_type_for_path(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name, value).unwrap()
}
