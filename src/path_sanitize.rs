use std::path::{Path, PathBuf};

pub fn sanitize_path(root: &Path, path: &str) -> Result<PathBuf, ()> {
    let path = Path::new(path);

    for component in path.components() {
        let os_str = component.as_os_str();
        if os_str == ".." || os_str.to_str().map(|s| s.starts_with('.')).unwrap_or(false) {
            return Err(());
        }
    }

    let full_path = root.join(path.strip_prefix("/").unwrap_or(path));
    let canonical = full_path.canonicalize().map_err(|_| ())?;

    if !canonical.starts_with(root) {
        return Err(());
    }

    Ok(canonical)
}
