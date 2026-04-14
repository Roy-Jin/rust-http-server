use std::time::Instant;

use axum::{
    extract::State,
    http::{HeaderMap, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    Router,
};
use clap::Parser;
use colored::*;
use percent_encoding::percent_decode_str;
use tokio::fs;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};

mod args;
mod dir_listing;
mod error_page;
mod file_server;
mod logging;
mod path_sanitize;
mod startup;
mod state;

use args::Args;
use dir_listing::serve_dir_listing;
use error_page::not_found_response;
use file_server::serve_file;
use logging::log_request;
use path_sanitize::sanitize_path;
use startup::print_startup_info;
use state::AppState;

fn print_author_info() {
    let authors = env!("CARGO_PKG_AUTHORS");
    println!("{}", format!(" Created by {} ", authors).white().bold().on_green());
    println!();
}

#[tokio::main]
async fn main() {
    print_author_info();
    
    let args = Args::parse();
    
    if args.help {
        println!("http-server v{}", env!("CARGO_PKG_VERSION"));
        println!();
        println!("USAGE:");
        println!("    http-server [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    -r, --root <ROOT>           Root directory to serve [default: .]");
        println!("    -a, --address <ADDRESS>     Address to bind [default: 0.0.0.0]");
        println!("    -p, --port <PORT>           Port number to listen [default: 80]");
        println!("        --no-dir-listing        Disable directory listings");
        println!("        --no-cors               Disable CORS");
        println!("        --no-gzip               Disable GZIP compression");
        println!("        --not-found-page <PATH> Custom 404 page [default: 404.html]");
        println!("    -o, --open                  Open browser automatically");
        println!("    -h, --help                  Print help information");
        println!("    -v, --version               Print version information");
        println!();
        return;
    }
    
    if args.version {
        println!("http-server v{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    
    let root = args.root.canonicalize().expect("Failed to canonicalize root path");

    let state = AppState {
        root: root.clone(),
        no_dir_listing: args.no_dir_listing,
        not_found_page: args.not_found_page.clone(),
    };

    let mut app = Router::new()
        .fallback(handle_request)
        .with_state(state);

    if !args.no_cors {
        app = app.layer(CorsLayer::new().allow_origin(Any));
    }

    if !args.no_gzip {
        app = app.layer(CompressionLayer::new());
    }

    let addr = format!("{}:{}", args.address, args.port);
    let listener_result = tokio::net::TcpListener::bind(&addr).await;

    let listener = match listener_result {
        Ok(l) => l,
        Err(e) if args.port == 80 => {
            println!("Port 80 requires elevated privileges. Trying port 8080 instead...");
            let addr_8080 = format!("{}:8080", args.address);
            tokio::net::TcpListener::bind(&addr_8080).await.expect("Failed to bind to port 8080")
        }
        Err(e) => panic!("Failed to bind to {}: {}", addr, e),
    };

    let local_addr = listener.local_addr().unwrap();
    print_startup_info(&args, &args.root, &root, local_addr);

    if args.open {
        let url = format!("http://localhost:{}", local_addr.port());
        if let Err(e) = open::that(&url) {
            eprintln!("Failed to open browser: {}", e);
        }
    }

    axum::serve(listener, app).await.expect("Server failed");
}

async fn handle_request(
    State(state): State<AppState>,
    method: Method,
    uri: Uri,
) -> Response {
    let start = Instant::now();
    let path = uri.path();
    let method_str = method.as_str();
    let uri_str = uri.to_string();

    let decoded_path = match percent_decode_str(path).decode_utf8() {
        Ok(p) => p,
        Err(_) => {
            let (status, resp) = not_found_response(&state).await;
            log_request(method_str, &uri_str, status, start);
            return resp;
        }
    };

    let file_path = match sanitize_path(&state.root, &decoded_path) {
        Ok(p) => p,
        Err(_) => {
            let (status, resp) = not_found_response(&state).await;
            log_request(method_str, &uri_str, status, start);
            return resp;
        }
    };

    let metadata = match fs::metadata(&file_path).await {
        Ok(m) => m,
        Err(_) => {
            let (status, resp) = not_found_response(&state).await;
            log_request(method_str, &uri_str, status, start);
            return resp;
        }
    };

    if metadata.is_dir() {
        let index_path = file_path.join("index.html");
        if index_path.exists() {
            if !path.ends_with('/') {
                let mut headers = HeaderMap::new();
                let new_path = format!("{}/", path);
                headers.insert("Location", new_path.parse().unwrap());
                let resp = (StatusCode::MOVED_PERMANENTLY, headers).into_response();
                log_request(method_str, &uri_str, StatusCode::MOVED_PERMANENTLY, start);
                return resp;
            }
            let (status, resp, _) = serve_file(&index_path, start).await;
            log_request(method_str, &uri_str, status, start);
            return resp;
        }

        if !state.no_dir_listing {
            if !path.ends_with('/') {
                let mut headers = HeaderMap::new();
                let new_path = format!("{}/", path);
                headers.insert("Location", new_path.parse().unwrap());
                let resp = (StatusCode::MOVED_PERMANENTLY, headers).into_response();
                log_request(method_str, &uri_str, StatusCode::MOVED_PERMANENTLY, start);
                return resp;
            }
            let resp = serve_dir_listing(&state.root, &file_path, &decoded_path).await;
            log_request(method_str, &uri_str, StatusCode::OK, start);
            return resp;
        }
    } else if metadata.is_file() {
        let (status, resp, _) = serve_file(&file_path, start).await;
        log_request(method_str, &uri_str, status, start);
        return resp;
    }

    let (status, resp) = not_found_response(&state).await;
    log_request(method_str, &uri_str, status, start);
    resp
}
