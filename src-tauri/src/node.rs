use crate::platform::get_metadata;
use crate::utils::is_filtered_out_due_to_invert_regex;
use crate::utils::is_filtered_out_due_to_regex;

use regex::Regex;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub struct Node {
    pub name: PathBuf,
    pub size: u64,
    pub children: Vec<Node>,
    pub inode_device: Option<(u64, u64)>,
    pub depth: usize,
}

#[allow(clippy::too_many_arguments)]
pub fn build_node(
    dir: PathBuf,
    children: Vec<Node>,
    filter_regex: &[Regex],
    invert_filter_regex: &[Regex],
    use_apparent_size: bool,
    is_symlink: bool,
    depth: usize,
) -> Option<Node> {
    get_metadata(&dir, use_apparent_size).map(|data| {
        let inode_device = if is_symlink && !use_apparent_size {
            None
        } else {
            data.1
        };

        let size = if is_filtered_out_due_to_regex(filter_regex, &dir)
            || is_filtered_out_due_to_invert_regex(invert_filter_regex, &dir)
            || (is_symlink && !use_apparent_size)
        {
            0
        } else {
            data.0
        };

        Node {
            name: dir,
            size,
            children,
            inode_device,
            depth,
        }
    })
}
