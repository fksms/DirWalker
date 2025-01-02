use std::{fs, path::Path};

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
