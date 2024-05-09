use crate::progress::RuntimeErrors;
use crate::dir_walker::WalkData;
use crate::progress::PIndicator;
use crate::dir_walker::walk_it;
use crate::utils::get_filesystem_devices;
use crate::utils::simplify_dir_names;
use crate::node::Node;

use std::collections::HashSet;
use std::panic;
use std::sync::Arc;
use std::sync::Mutex;
use sysinfo::{System, SystemExt};
use regex::Regex;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WalkParams {
    pub target_directories: Option<Vec<String>>,
    pub regex_filter: Option<Vec<String>>,
    pub regex_invert_filter: Option<Vec<String>>,
    pub ignore_directories: Option<Vec<String>>,
    pub by_filecount: bool,
    pub limit_filesystem: bool,
    pub dereference_links: bool,
    pub ignore_hidden_files: bool,
    pub use_apparent_size: bool,
}

pub fn exec_dust(walk_params: WalkParams) -> Option<Vec<Node>> {

    // エラー格納用
    let errors = RuntimeErrors::default();
    let error_listen_for_ctrlc = Arc::new(Mutex::new(errors));
    let errors_for_rayon = error_listen_for_ctrlc.clone();
    let errors_final = error_listen_for_ctrlc.clone();

    // Ctrl-Cで中止
    ctrlc::set_handler(move || {
        error_listen_for_ctrlc.lock().unwrap().abort = true;
        println!("\nAborting");
    })
    .expect("Error setting Ctrl-C handler");
    
    // 以下パラメータ設定
    let target_dirs = match walk_params.target_directories {
        Some(values) => values,
        None => vec![String::from("."); 0],
    };

    let filter_regexs = match walk_params.regex_filter {
        Some(values) => values
            .iter()
            .map(|reg| Regex::new(reg).unwrap())
            .collect::<Vec<Regex>>(),
        None => vec![],
    };

    let invert_filter_regexs = match walk_params.regex_invert_filter {
        Some(values) => values
            .iter()
            .map(|reg| Regex::new(reg).unwrap())
            .collect::<Vec<Regex>>(),
        None => vec![],
    };

    let ignore_directories = match walk_params.ignore_directories {
        Some(values) => values
            .iter()
            .map(PathBuf::from)
            .collect::<Vec<PathBuf>>(),
        None => vec![],
    };

    let by_filecount = walk_params.by_filecount;
    let limit_filesystem = walk_params.limit_filesystem;
    let follow_links = walk_params.dereference_links;

    let simplified_dirs = simplify_dir_names(target_dirs);
    let allowed_filesystems = limit_filesystem
        .then(|| get_filesystem_devices(simplified_dirs.iter()))
        .unwrap_or_default();

    let ignored_full_path: HashSet<PathBuf> = ignore_directories
        .into_iter()
        .flat_map(|x| simplified_dirs.iter().map(move |d| d.join(&x)))
        .collect();

    let ignore_hidden = walk_params.ignore_hidden_files;

    let indicator = PIndicator::build_me();

    let walk_data = WalkData {
        ignore_directories: ignored_full_path,
        filter_regex: &filter_regexs,
        invert_filter_regex: &invert_filter_regexs,
        allowed_filesystems,
        use_apparent_size: walk_params.use_apparent_size,
        by_filecount,
        ignore_hidden,
        follow_links,
        progress_data: indicator.data.clone(),
        errors: errors_for_rayon,
    };
    let stack_size = None;
    init_rayon(&stack_size);

    // Walk
    let top_level_nodes = walk_it(simplified_dirs, &walk_data);

    // 強制終了
    if errors_final.lock().unwrap().abort {
        return None;
    }

    // 以下エラー出力
    let final_errors = walk_data.errors.lock().unwrap();
    if !final_errors.file_not_found.is_empty() {
        let err = final_errors
            .file_not_found
            .iter()
            .map(|a| a.as_ref())
            .collect::<Vec<&str>>()
            .join(", ");
        eprintln!("No such file or directory: {}", err);
    }
    if final_errors.no_permissions {
        eprintln!("Did not have permissions for all directories");
    }
    if !final_errors.unknown_error.is_empty() {
        let err = final_errors
            .unknown_error
            .iter()
            .map(|a| a.as_ref())
            .collect::<Vec<&str>>()
            .join(", ");
        eprintln!("Unknown Error: {}", err);
    }

    // ノード出力
    return Some(top_level_nodes);
}

fn init_rayon(stack_size: &Option<usize>) {
    // Rayon seems to raise this error on 32-bit builds
    // The global thread pool has not been initialized.: ThreadPoolBuildError { kind: GlobalPoolAlreadyInitialized }
    if cfg!(target_pointer_width = "64") {
        let result = panic::catch_unwind(|| {
            match stack_size {
                Some(n) => rayon::ThreadPoolBuilder::new()
                    .stack_size(*n)
                    .build_global(),
                None => {
                    let large_stack = usize::pow(1024, 3);
                    let mut s = System::new();
                    s.refresh_memory();
                    let available = s.available_memory();

                    if available > large_stack.try_into().unwrap() {
                        // Larger stack size to handle cases with lots of nested directories
                        rayon::ThreadPoolBuilder::new()
                            .stack_size(large_stack)
                            .build_global()
                    } else {
                        rayon::ThreadPoolBuilder::new().build_global()
                    }
                }
            }
        });
        if result.is_err() {
            eprintln!("Problem initializing rayon, try: export RAYON_NUM_THREADS=1")
        }
    }
}
