use std::path::Path;

pub fn join_path(dirs: &[String]) -> Result<String, &'static str> {
    if dirs.is_empty() {
        return Err("No path provided");
    }

    let path = dirs.join("/");

    if Path::new(&path).try_exists();
}
