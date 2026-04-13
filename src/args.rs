use std::path::PathBuf;

pub use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(about, long_about = None, disable_help_flag = true, disable_version_flag = true)]
pub struct Args {
    #[arg(short, long, default_value = ".")]
    pub root: PathBuf,

    #[arg(short, long, default_value = "0.0.0.0")]
    pub address: String,

    #[arg(short, long, default_value_t = 80)]
    pub port: u16,

    #[arg(long)]
    pub no_dir_listing: bool,

    #[arg(long)]
    pub no_cors: bool,

    #[arg(long)]
    pub no_gzip: bool,

    #[arg(long, default_value = "404.html")]
    pub not_found_page: PathBuf,

    #[arg(short, long)]
    pub help: bool,

    #[arg(short, long)]
    pub version: bool,
}
