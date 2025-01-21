use crate::dir_walker::walk_it;
use crate::dir_walker::WalkData;
use crate::node::Node;
use crate::progress::ErrorHandler;
use crate::progress::ProgressHandler;
use crate::progress::ORDERING;
use crate::progress::{indicator_spawn, indicator_stop};
use crate::utils::normalize_path;

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::panic;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use sysinfo::{System, SystemExt};
use tauri::Emitter;

// Rayonのスタックサイズ
const STACK_SIZE_OF_RAYON: usize = 1024 * 1024 * 1024; // Set stack size to 1024MB

#[derive(Debug, Deserialize, Serialize)]
pub struct WalkParams {
    pub target_directory: String,
    pub regex_filter: Option<Vec<String>>,
    pub regex_invert_filter: Option<Vec<String>>,
    pub ignore_directories: Option<Vec<String>>,
    pub use_apparent_size: bool,
}

pub fn init_walk(
    walk_params: WalkParams,
    errors: &Arc<Mutex<ErrorHandler>>,
    progress: &Arc<ProgressHandler>,
    app: tauri::AppHandle,
) -> Option<Node> {
    // エラー格納用
    let errors_for_rayon = errors.clone();
    let errors_final = errors.clone();

    // appのクローン
    let app_clone = app.clone();

    // 以下パラメータ設定
    let simplified_dir = normalize_path(walk_params.target_directory);

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
        Some(values) => values.iter().map(PathBuf::from).collect::<Vec<PathBuf>>(),
        None => vec![],
    };

    let ignored_full_path: HashSet<PathBuf> = ignore_directories
        .into_iter()
        .map(|x| simplified_dir.join(&x))
        .collect();

    let walk_data = WalkData {
        ignore_directories: ignored_full_path,
        filter_regex: &filter_regexs,
        invert_filter_regex: &invert_filter_regexs,
        use_apparent_size: walk_params.use_apparent_size,
        progress_data: progress.clone(),
        errors: errors_for_rayon,
    };

    // Rayonスレッドを作成
    init_rayon();

    // Progressを表示
    let indicator_handler = indicator_spawn(progress, app_clone);

    // Walk
    let top_level_node = walk_it(simplified_dir, &walk_data);

    // Progressを終了
    indicator_stop(indicator_handler);

    // ステータスを更新
    let prog_data = progress.clone();
    prog_data.scan_complete.store(true, ORDERING);

    let encode_result: Result<String, _> = serde_json::to_string(&prog_data);
    match encode_result {
        // 正常にエンコードできた場合
        Ok(str) => {
            // WebViewに送信
            app.emit("ProgressNotification", str).unwrap();
        }
        // エンコードに失敗した場合
        Err(_) => println!("Progress encode error."),
    }

    // 強制終了
    if errors_final.lock().unwrap().abort {
        println!("Aborting");
        return None;
    }

    // エラー出力
    let final_errors = walk_data.errors.lock().unwrap();
    if final_errors.no_permissions {
        eprintln!("Did not have permissions for all directories");
    }
    if !final_errors.file_not_found.is_empty() {
        let err = final_errors
            .file_not_found
            .iter()
            .map(|a| a.as_ref())
            .collect::<Vec<&str>>()
            .join(", ");
        eprintln!("No such file or directory: {}", err);
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
    return top_level_node;
}

fn init_rayon() {
    // Rayon seems to raise this error on 32-bit builds
    // The global thread pool has not been initialized.: ThreadPoolBuildError { kind: GlobalPoolAlreadyInitialized }
    if cfg!(target_pointer_width = "64") {
        let result = panic::catch_unwind(|| {
            let large_stack = STACK_SIZE_OF_RAYON;
            let mut s = System::new();
            s.refresh_memory();
            let available = s.available_memory();

            if available > u64::try_from(large_stack).unwrap() {
                // Larger stack size to handle cases with lots of nested directories
                rayon::ThreadPoolBuilder::new()
                    .stack_size(large_stack)
                    .build_global()
            } else {
                // Do not specify stack size
                rayon::ThreadPoolBuilder::new().build_global()
            }
        });
        if result.is_err() {
            eprintln!("Problem initializing rayon, try: export RAYON_NUM_THREADS=1")
        }
    }
}
