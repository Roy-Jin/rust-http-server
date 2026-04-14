# HTTP-SERVER

A lightweight HTTP server written in Rust. This project aims to provide a simple and efficient file server for local development and testing.

[English](README.md) | [中文](README_zh.md)

## Features

- 🚀 **Fast & Async**: Built with [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/) for high performance
- 📁 **Directory Listing**: Beautiful, modern UI for browsing directories
- 🔒 **Path Sanitization**: Secure path handling to prevent directory traversal
- 🌐 **CORS Support**: Cross-Origin Resource Sharing enabled by default
- 📦 **GZIP Compression**: Automatic compression for smaller payloads
- 📄 **Custom 404 Pages**: Serve custom 404.html files
- 🎨 **Colorful CLI**: Beautiful terminal output with colored status messages
- 🌍 **Auto Open Browser**: Automatically open the browser when server starts

## Installation

### From Source

```bash
git clone https://github.com/Roy-Jin/rust-http-server.git
cd rust-http-server
cargo build --release
```

The compiled binary will be at `target/release/http-server`.

## Usage

### Basic Usage

Start serving the current directory on port 80:

```bash
.\http-server
```

### Custom Port & Directory

```bash
.\http-server -p 8080 -r ./public
```

### Command Line Options

```
USAGE:
    http-server [OPTIONS]

OPTIONS:
    -r, --root <ROOT>           Root directory to serve [default: .]
    -a, --address <ADDRESS>     Address to bind [default: 0.0.0.0]
    -p, --port <PORT>           Port number to listen [default: 80]
        --no-dir-listing        Disable directory listings
        --no-cors               Disable CORS
        --no-gzip               Disable GZIP compression
        --not-found-page <PATH> Custom 404 page [default: 404.html]
    -o, --open                  Open browser automatically
    -h, --help                  Print help information
    -v, --version               Print version information
```

## Examples

### Serve a specific directory

```bash
.\http-server -r /path/to/your/directory
```

### Disable directory listing

```bash
.\http-server --no-dir-listing
```

### Disable CORS

```bash
.\http-server --no-cors
```

### Use custom 404 page

```bash
.\http-server --not-found-page my-404.html
```

### Auto open browser

```bash
.\http-server -o
```

## How It Works

1. **File Serving**: Serves static files from the specified root directory
2. **Index Files**: Automatically serves `index.html` if it exists in a directory
3. **Directory Listing**: When no index.html is found, shows a beautiful directory browser
4. **Path Handling**: Sanitizes all paths to prevent directory traversal attacks
5. **Compression**: Automatically compresses responses with GZIP when supported
6. **Logging**: Logs all requests with status codes and response times

## Tech Stack

- **[Axum](https://github.com/tokio-rs/axum)**: Web framework
- **[Tokio](https://tokio.rs/)**: Async runtime
- **[tower-http](https://github.com/tower-rs/tower-http)**: HTTP middleware (CORS, compression)
- **[clap](https://github.com/clap-rs/clap)**: Command line argument parsing
- **[colored](https://github.com/mackwic/colored)**: Terminal colors
- **[open](https://github.com/Byron/open-rs)**: Open browser automatically

## License

This project is licensed under the [MIT License](LICENSE).

## Author

Created by [Roy-Jin](https://github.com/Roy-Jin)
