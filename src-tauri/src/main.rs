// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dir_walker;
mod dust;
mod node;
mod platform;
mod progress;
mod utils;
mod walk_manager;

use crate::dust::WalkParams;
use crate::dust::exec_dust;
use crate::node::Node;
use crate::walk_manager::WalkManager;

use tauri::Manager;

// ノードをjsonに変換
pub fn nodes_to_json(nodes: Option<Vec<Node>>) -> Result<String, String> {
    match nodes {
        // ノードに要素がある場合
        Some(nodes) => {
            println!("Node found.");

            let encode_result: Result<String, _> = serde_json::to_string(&nodes);

            match encode_result {
                // 正常にノードデータをエンコードできた場合
                Ok(str_nodes) => {
                    println!("Encode done.");
                    Ok(str_nodes)
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

// Walk Start
#[tauri::command(rename_all = "snake_case")]
async fn walk_start(state: tauri::State<'_, WalkManager>, str_params: &str) -> Result<String, String> {

    // 終了フラグをfalseに設定
    state.set_abort_flag(false);
    
    let decode_result: Result<WalkParams, _> = serde_json::from_str(&str_params);
    println!("Receive Params.");

    match decode_result {
        // 正常にパラメータをデコードできた場合
        Ok(walk_params) => {
            println!("Decode done.");
            let walk_data = exec_dust(walk_params, state.get_error_handler());
            println!("Walk done.");

            // ノードをセット
            state.set_nodes(walk_data);

            // 深さ指定で部分的ノードを取得
            let depth = 3;
            let partial_nodes = state.get_partial_nodes(depth);

            // ノードをjsonに変換
            return nodes_to_json(partial_nodes);
        }
        // パラメータのデコードに失敗した場合
        Err(err) => {
            println!("{}", err.to_string());
            Err(err.to_string())
        }
    }
}

// ノードをリロード
#[tauri::command(rename_all = "snake_case")]
fn node_reload(state: tauri::State<'_, WalkManager>) -> Result<String, String> {

    // 深さ指定で部分的ノードを取得
    let depth = 3;
    let partial_nodes = state.get_partial_nodes(depth);

    // ノードをjsonに変換
    return nodes_to_json(partial_nodes);
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
