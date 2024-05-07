use platform::get_metadata;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::platform;
use regex::Regex;

pub fn simplify_dir_names<P: AsRef<Path>>(filenames: Vec<P>) -> HashSet<PathBuf> {
    let mut top_level_names: HashSet<PathBuf> = HashSet::with_capacity(filenames.len());

    for t in filenames {
        let top_level_name = normalize_path(t);
        let mut can_add = true;
        let mut to_remove: Vec<PathBuf> = Vec::new();

        for tt in top_level_names.iter() {
            if is_a_parent_of(&top_level_name, tt) {
                to_remove.push(tt.to_path_buf());
            } else if is_a_parent_of(tt, &top_level_name) {
                can_add = false;
            }
        }
        for r in to_remove {
            top_level_names.remove(&r);
        }
        if can_add {
            top_level_names.insert(top_level_name);
        }
    }

    top_level_names
}

pub fn get_filesystem_devices<'a, P: IntoIterator<Item = &'a PathBuf>>(paths: P) -> HashSet<u64> {
    // Gets the device ids for the filesystems which are used by the argument paths
    paths
        .into_iter()
        .filter_map(|p| match get_metadata(p, false) {
            Some((_size, Some((_id, dev)))) => Some(dev),
            _ => None,
        })
        .collect()
}

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

fn is_a_parent_of<P: AsRef<Path>>(parent: P, child: P) -> bool {
    let parent = parent.as_ref();
    let child = child.as_ref();
    child.starts_with(parent) && !parent.starts_with(child)
}
