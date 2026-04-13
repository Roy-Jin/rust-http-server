use std::net::SocketAddr;
use std::path::Path;

use colored::*;

use crate::args::Args;

pub fn print_startup_info(args: &Args, relative_root: &Path, _absolute_root: &Path, addr: SocketAddr) {
    println!("\nStarting up http-server, serving {}\n", relative_root.display().to_string().cyan());
    
    println!("{} {}", "http-server version:".green(), env!("CARGO_PKG_VERSION").bold());
    println!();
    println!("{}", "http-server settings:".green());
    
    let cors_status = if args.no_cors { "disabled".red() } else { "enabled".green() };
    println!("  CORS: {}", cors_status);
    
    let dir_listing_status = if args.no_dir_listing { "hidden".red() } else { "visible".green() };
    println!("  Directory Listings: {}", dir_listing_status);
    
    let gzip_status = if args.no_gzip { "false".red() } else { "true".green() };
    println!("  Serve GZIP Files: {}", gzip_status);
    println!();
    println!("{}", "Available on:".green().bold());
    
    if args.address == "0.0.0.0" {
        if let Ok(ifaces) = get_if_addrs::get_if_addrs() {
            for iface in ifaces {
                if !iface.is_loopback() && iface.ip().is_ipv4() {
                    let url = format!("http://{}:{}", iface.ip(), addr.port());
                    println!("  {}", url.cyan());
                }
            }
        }
        println!("  {}", format!("http://127.0.0.1:{}", addr.port()).cyan());
        println!("  {}", format!("http://localhost:{}", addr.port()).cyan());
    } else {
        println!("  {}", format!("http://{}", addr).cyan());
    }
    
    println!();
    println!("Hit {} to stop the server\n", "CTRL-C".yellow().bold());
}
