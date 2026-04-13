use std::path::PathBuf;

#[derive(Clone)]
pub struct AppState {
    pub root: PathBuf,
    pub no_dir_listing: bool,
    pub not_found_page: PathBuf,
}
