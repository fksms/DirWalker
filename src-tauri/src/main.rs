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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Walk Start
#[tauri::command(rename_all = "snake_case")]
fn walk_start(state: tauri::State<'_, WalkManager>, str_params: &str) -> Result<String, String> {
    
    let decode_result: Result<WalkParams, _> = serde_json::from_str(&str_params);
    println!("Receive Params.");

    match decode_result {
        // 正常にパラメータをデコードできた場合
        Ok(walk_params) => {
            println!("Decode done.");
            let walk_data = exec_dust(walk_params);
            println!("Walk done.");

            // ノードをセット
            state.set_nodes(walk_data.clone());

            // ノードをjsonに変換
            return nodes_to_json(walk_data);
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

    // ノードを取得
    let nodes = state.get_nodes();

    // ノードをjsonに変換
    return nodes_to_json(nodes);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            walk_start,
            node_reload,
        ])
        .setup(|app|{
            let walk_manager = WalkManager::new();
            app.manage(walk_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
