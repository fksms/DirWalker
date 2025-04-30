mod dir_walker;
mod frontend_utils;
mod init_walk;
mod node;
mod platform;
mod progress;
mod utils;
mod walk_manager;

use tauri::Manager;

use crate::frontend_utils::{
    check_full_disk_access_permission, move_to_trash, open_file_manager, remove_file_or_directory,
};
use crate::init_walk::init_walk;
use crate::init_walk::WalkParams;
use crate::node::Node;
use crate::walk_manager::WalkManager;

// ノードをjsonに変換
pub fn node_to_json(node: Option<Node>) -> Result<String, String> {
    match node {
        // ノードに要素がある場合
        Some(node) => {
            println!("Node found.");

            let encode_result: Result<String, _> = serde_json::to_string(&node);

            match encode_result {
                // 正常にノードデータをエンコードできた場合
                Ok(str_node) => {
                    println!("Encode done.");
                    Ok(str_node)
                }
                // ノードデータのエンコードに失敗した場合
                Err(err) => Err(err.to_string()),
            }
        }
        // ノードが空の場合
        None => {
            println!("Node is empty.");
            Ok("".to_string())
        }
    }
}

// Walk Start（asyncで非同期とする）
#[tauri::command(rename_all = "snake_case")]
async fn walk_start(
    str_params: &str,
    state: tauri::State<'_, WalkManager>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    // 終了フラグをfalseに設定
    state.set_abort_flag(false);

    let decode_result: Result<WalkParams, _> = serde_json::from_str(&str_params);
    println!("Receive Params.");

    match decode_result {
        // 正常にパラメータをデコードできた場合
        Ok(walk_params) => {
            println!("Decode done.");

            // Walk
            let walk_data = init_walk(
                walk_params,
                state.get_error_handler(),
                state.get_progress_handler(),
                app,
            );
            println!("Walk done.");

            // 現状使用しないので無効化
            /*
            // Walk Dataのクローン
            let walk_data_clone = walk_data.clone();

            // ノードをセット
            state.set_node(walk_data_clone);
            */

            // ノードをjsonに変換
            return node_to_json(walk_data);
        }
        // パラメータのデコードに失敗した場合
        Err(err) => {
            println!("{}", err.to_string());
            Err(err.to_string())
        }
    }
}

// 現状使用しないので無効化
/*
// ノードをリロード（asyncで非同期とする）
#[tauri::command(rename_all = "snake_case")]
async fn node_reload(state: tauri::State<'_, WalkManager>) -> Result<String, String> {
    // ノードを取得
    let node = state.get_node();

    // ノードをjsonに変換
    return node_to_json(node);
}
*/

// 強制終了
#[tauri::command(rename_all = "snake_case")]
fn abort(state: tauri::State<'_, WalkManager>) {
    // 終了フラグをtrueに設定
    state.set_abort_flag(true);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(move |app| {
            let walk_manager = WalkManager::new();
            app.manage(walk_manager);

            // MacOSのみメニューを生成
            #[cfg(target_os = "macos")]
            {
                use tauri::menu::{AboutMetadata, Menu, SubmenuBuilder};

                let menu = Menu::new(app)?;

                let mut submenu = SubmenuBuilder::new(app, "File")
                    .about_with_text("About", Some(AboutMetadata::default()))
                    .separator()
                    .quit_with_text("Quit")
                    .build()?;

                menu.append(&submenu)?;

                submenu = SubmenuBuilder::new(app, "Edit")
                    .undo()
                    .redo()
                    .separator()
                    .cut()
                    .copy()
                    .paste()
                    .select_all()
                    .build()?;

                menu.append(&submenu)?;

                submenu = SubmenuBuilder::new(app, "Window")
                    .minimize()
                    .separator()
                    .close_window()
                    .build()?;

                menu.append(&submenu)?;

                app.set_menu(menu)?;
            }

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            walk_start,
            abort,
            remove_file_or_directory,
            open_file_manager,
            check_full_disk_access_permission,
            move_to_trash
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
