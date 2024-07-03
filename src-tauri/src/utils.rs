use std::path::{Path, PathBuf};

use regex::Regex;

pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    // normalize path ...
    // 1. removing repeated separators
    // 2. removing interior '.' ("current directory") path segments
    // 3. removing trailing extra separators and '.' ("current directory") path segments
    // * `Path.components()` does all the above work; ref: <https://doc.rust-lang.org/std/path/struct.Path.html#method.components>
    // 4. changing to os preferred separator (automatically done by recollecting components back into a PathBuf)
    path.as_ref().components().collect()
}

pub fn is_filtered_out_due_to_regex(filter_regex: &[Regex], dir: &Path) -> bool {
    if filter_regex.is_empty() {
        false
    } else {
        filter_regex
            .iter()
            .all(|f| !f.is_match(&dir.as_os_str().to_string_lossy()))
    }
}

pub fn is_filtered_out_due_to_invert_regex(filter_regex: &[Regex], dir: &Path) -> bool {
    filter_regex
        .iter()
        .any(|f| f.is_match(&dir.as_os_str().to_string_lossy()))
}
