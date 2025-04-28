use opener::open;
use std::{fs, path::Path};

use tauri::Manager;

// ファイルマネージャーを開く
#[tauri::command(rename_all = "snake_case")]
pub async fn open_file_manager(path: String) -> Result<(), String> {
    let result = open(path);

    match result {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err.to_string()),
    }
}

// ファイルorディレクトリを削除する
#[tauri::command(rename_all = "snake_case")]
pub async fn remove_file_or_directory(path: String) -> Result<(), String> {
    // ディレクトリの場合
    if Path::new(&path).is_dir() {
        match fs::remove_dir_all(path) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err.to_string()),
        }
    }
    // ファイルの場合
    else {
        match fs::remove_file(path) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err.to_string()),
        }
    }
}

// フルディスクアクセスの権限を確認する
// 参照: https://github.com/ayangweb/tauri-plugin-macos-permissions/blob/c025ab4ad762060033b5e1fc2181e2b8ff50c91c/src/commands.rs#L63
#[tauri::command(rename_all = "snake_case")]
pub async fn check_full_disk_access_permission(app: tauri::AppHandle) -> bool {
    #[cfg(target_os = "macos")]
    {
        // Reference: https://github.com/inket/FullDiskAccess/blob/846e04ea2b84fce843f47d7e7f3421189221829c/Sources/FullDiskAccess/FullDiskAccess.swift#L46
        let check_dirs = vec!["Library/Containers/com.apple.stocks", "Library/Safari"];

        if let Ok(home_dir) = app.path().home_dir() {
            for check_dir in check_dirs.iter() {
                if fs::read_dir(&home_dir.join(check_dir)).is_ok() {
                    return true;
                }
            }
        }

        return false;
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = app;

        return true;
    }
}
