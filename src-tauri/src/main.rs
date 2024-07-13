// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dir_walker;
mod init_walk;
mod node;
mod platform;
mod progress;
mod utils;
mod walk_manager;

use opener::open;
use tauri::Manager;
use tauri::{Menu, MenuItem, Submenu};

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

            // Walk Dataのクローン
            let walk_data_clone = walk_data.clone();

            // ノードをセット
            state.set_node(walk_data_clone);

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

// ノードをリロード（asyncで非同期とする）
#[tauri::command(rename_all = "snake_case")]
async fn node_reload(state: tauri::State<'_, WalkManager>) -> Result<String, String> {
    // ノードを取得
    let node = state.get_node();

    // ノードをjsonに変換
    return node_to_json(node);
}

// 強制終了
#[tauri::command(rename_all = "snake_case")]
fn abort(state: tauri::State<'_, WalkManager>) {
    // 終了フラグをtrueに設定
    state.set_abort_flag(true);
}

// ファイルマネージャーを開く
#[tauri::command(rename_all = "snake_case")]
async fn open_file_manager(path: String) -> Result<(), String> {
    let result = open(path);

    match result {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err.to_string()),
    }
}

fn main() {
    let context = tauri::generate_context!();

    // --------------------ここからカスタムメニュー--------------------
    let mut menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
        use tauri::AboutMetadata;
        menu = menu.add_submenu(Submenu::new(
            context.package_info().name.clone(), // アプリケーション名
            Menu::new()
                .add_native_item(MenuItem::About(
                    context.package_info().name.clone(), // アプリケーション名
                    AboutMetadata::default(),
                ))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Services)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ));
    }

    #[cfg(not(target_os = "macos"))]
    {
        let mut file_menu = Menu::new();
        file_menu = file_menu.add_native_item(MenuItem::Quit);
        menu = menu.add_submenu(Submenu::new("File", file_menu));
    }

    #[cfg(not(target_os = "linux"))]
    let mut edit_menu = Menu::new();
    /*
    #[cfg(target_os = "macos")]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::Undo);
        edit_menu = edit_menu.add_native_item(MenuItem::Redo);
        edit_menu = edit_menu.add_native_item(MenuItem::Separator);
    }
    */
    #[cfg(not(target_os = "linux"))]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::Cut);
        edit_menu = edit_menu.add_native_item(MenuItem::Copy);
        edit_menu = edit_menu.add_native_item(MenuItem::Paste);
    }
    #[cfg(target_os = "macos")]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::SelectAll);
    }
    #[cfg(not(target_os = "linux"))]
    {
        menu = menu.add_submenu(Submenu::new("Edit", edit_menu));
    }
    /*
    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            "View",
            Menu::new().add_native_item(MenuItem::EnterFullScreen),
        ));
    }
    */

    let mut window_menu = Menu::new();
    window_menu = window_menu.add_native_item(MenuItem::Minimize);
    /*
    #[cfg(target_os = "macos")]
    {
        window_menu = window_menu.add_native_item(MenuItem::Zoom);
        window_menu = window_menu.add_native_item(MenuItem::Separator);
    }
    */
    window_menu = window_menu.add_native_item(MenuItem::CloseWindow);

    menu = menu.add_submenu(Submenu::new("Window", window_menu));
    // --------------------ここまでカスタムメニュー--------------------

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            walk_start,
            node_reload,
            abort,
            open_file_manager
        ])
        .setup(|app| {
            let walk_manager = WalkManager::new();
            app.manage(walk_manager);
            Ok(())
        })
        // コンテキストメニュー表示用
        .plugin(tauri_plugin_context_menu::init())
        // メニューバーをカスタマイズ
        .menu(menu)
        .run(context)
        .expect("error while running tauri application");
}
