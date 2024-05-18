use std::fs;
use std::sync::Arc;
use std::sync::Mutex;

use crate::node::Node;
use crate::progress::ProgressHandler;
use crate::progress::ErrorHandler;
use crate::progress::ORDERING;
use crate::utils::is_filtered_out_due_to_invert_regex;
use crate::utils::is_filtered_out_due_to_regex;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use regex::Regex;
use std::path::PathBuf;

use std::collections::HashSet;

use crate::node::build_node;
use std::fs::DirEntry;

pub struct WalkData<'a> {
    pub ignore_directories: HashSet<PathBuf>,
    pub filter_regex: &'a [Regex],
    pub invert_filter_regex: &'a [Regex],
    pub use_apparent_size: bool,
    pub progress_data: Arc<ProgressHandler>,
    pub errors: Arc<Mutex<ErrorHandler>>,
}

/* -------------------------------------------------------------------------- */

pub fn walk_it(dir: PathBuf, walk_data: &WalkData) -> Option<Node> {
    let mut inodes = HashSet::new();

    let prog_data = &walk_data.progress_data;
    prog_data.clear_state();

    match walk(dir, walk_data, 0) {
        Some(node) => {
            return clean_inodes(node, &mut inodes, walk_data.use_apparent_size);
        },
        None => {
            return None;
        }
    };
}

/* -------------------------------------------------------------------------- */

// Remove files which have the same inode, we don't want to double count them.
fn clean_inodes(x: Node, inodes: &mut HashSet<(u64, u64)>, use_apparent_size: bool) -> Option<Node> {
    if !use_apparent_size {
        if let Some(id) = x.inode_device {
            if !inodes.insert(id) {
                return None;
            }
        }
    }
    
    let new_children: Vec<_> = x.children
        .into_iter()
        .filter_map(|c| clean_inodes(c, inodes, use_apparent_size))
        .collect();

    Some(Node {
        name: x.name,
        size: x.size + new_children.iter().map(|c| c.size).sum::<u64>(),
        children: new_children,
        inode_device: None, // メモリ削減
        depth: x.depth,
    })
}

/* -------------------------------------------------------------------------- */

fn ignore_file(entry: &DirEntry, walk_data: &WalkData) -> bool {

    // Keeping `walk_data.filter_regex.is_empty()` is important for performance reasons, it stops unnecessary work
    if !walk_data.filter_regex.is_empty()
        && entry.path().is_file()
        && is_filtered_out_due_to_regex(walk_data.filter_regex, &entry.path())
    {
        return true;
    }

    if !walk_data.invert_filter_regex.is_empty()
        && entry.path().is_file()
        && is_filtered_out_due_to_invert_regex(walk_data.invert_filter_regex, &entry.path())
    {
        return true;
    }

    return walk_data.ignore_directories.contains(&entry.path());
}

fn walk(dir: PathBuf, walk_data: &WalkData, depth: usize) -> Option<Node> {
    let prog_data = &walk_data.progress_data;
    let errors = &walk_data.errors;

    // abortフラグでNoneをリターン
    if errors.lock().unwrap().abort {
        return None;
    }

    let children = if dir.is_dir() {
        // dirがディレクトリの場合
        let read_dir = fs::read_dir(&dir);
        match read_dir {
            Ok(entries) => {
                entries
                    .into_iter()
                    .par_bridge()
                    .filter_map(|entry| {
                        if let Ok(ref entry) = entry {
                            // uncommenting the below line gives simpler code but
                            // rayon doesn't parallelize as well giving a 3X performance drop
                            // hence we unravel the recursion a bit

                            // return walk(entry.path(), walk_data, depth)

                            if !ignore_file(entry, walk_data) {
                                if let Ok(data) = entry.file_type() {
                                    if data.is_dir()
                                    {
                                        return walk(entry.path(), walk_data, depth + 1);
                                    }

                                    let node = build_node(
                                        entry.path(),
                                        vec![],
                                        walk_data.filter_regex,
                                        walk_data.invert_filter_regex,
                                        walk_data.use_apparent_size,
                                        data.is_symlink(),
                                        depth + 1,
                                    );

                                    prog_data.num_files.fetch_add(1, ORDERING);
                                    if let Some(ref file) = node {
                                        prog_data.total_file_size.fetch_add(file.size, ORDERING);
                                    }

                                    return node;
                                }
                            }
                        } else {
                            let mut editable_error = errors.lock().unwrap();
                            editable_error.no_permissions = true
                        }
                        None
                    })
                    .collect()
            }
            Err(failed) => {
                let mut editable_error = errors.lock().unwrap();
                match failed.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        editable_error.no_permissions = true;
                    }
                    std::io::ErrorKind::NotFound => {
                        editable_error.file_not_found.insert(failed.to_string());
                    }
                    _ => {
                        editable_error.unknown_error.insert(failed.to_string());
                    }
                }
                vec![]
            }
        }
    } else {
        // dirがファイルの場合
        if !dir.is_file() {
            // dirがディレクトリでもファイルでも無い場合
            let mut editable_error = errors.lock().unwrap();
            let bad_file = dir.as_os_str().to_string_lossy().into();
            editable_error.file_not_found.insert(bad_file);
        }
        // 空配列をリターン
        vec![]
    };
    let node = build_node(
        dir,
        children,
        walk_data.filter_regex,
        walk_data.invert_filter_regex,
        walk_data.use_apparent_size,
        false,
        depth,
    );

    return node;
}
