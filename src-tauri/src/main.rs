// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dir_walker;
mod init_walk;
mod node;
mod platform;
mod progress;
mod utils;
mod walk_manager;

use tauri::Manager;

use crate::init_walk::WalkParams;
use crate::init_walk::init_walk;
use crate::node::Node;
use crate::walk_manager::WalkManager;

// 部分的ノードの深さ
const DEPTH_OF_PARTIAL_NODE: usize = 3;

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
                Err(err) => {
                    Err(err.to_string())
                }
            }
        },
        // ノードが空の場合
        None => {
            println!("Node is empty.");
            Ok("".to_string())
        }
    }
}

// Walk Start（asyncで非同期とする）
#[tauri::command(rename_all = "snake_case")]
async fn walk_start(str_params: &str, state: tauri::State<'_, WalkManager>, app: tauri::AppHandle) -> Result<String, String> {

    // 終了フラグをfalseに設定
    state.set_abort_flag(false);
    
    let decode_result: Result<WalkParams, _> = serde_json::from_str(&str_params);
    println!("Receive Params.");

    match decode_result {
        // 正常にパラメータをデコードできた場合
        Ok(walk_params) => {
            println!("Decode done.");

            // Walk
            let walk_data = init_walk(walk_params, state.get_error_handler(), state.get_progress_handler(), app);
            println!("Walk done.");

            // ノードをセット
            state.set_node(walk_data);

            // 深さ指定で部分的ノードを取得
            let partial_node = state.get_full_node();

            // ノードをjsonに変換
            return node_to_json(partial_node);
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

    // 深さ指定で部分的ノードを取得
    let partial_node = state.get_partial_node(DEPTH_OF_PARTIAL_NODE);

    // ノードをjsonに変換
    return node_to_json(partial_node);
}

// 強制終了
#[tauri::command(rename_all = "snake_case")]
fn abort(state: tauri::State<'_, WalkManager>) {

    // 終了フラグをtrueに設定
    state.set_abort_flag(true);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            walk_start,
            node_reload,
            abort,
        ])
        .setup(|app|{
            let walk_manager = WalkManager::new();
            app.manage(walk_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
