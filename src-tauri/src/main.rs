// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dir_walker;
mod dust;
mod node;
mod platform;
mod progress;
mod utils;

use crate::dust::DustParams;
use crate::dust::exec_dust;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
fn walk_start(str_params: &str) -> Result<String, String> {
    
    let decode_result: Result<DustParams, _> = serde_json::from_str(&str_params);
    println!("Receive Params.");

    match decode_result {
        // 正常にパラメータをデコードできた場合
        Ok(dust_params) => {
            println!("Decode done.");
            let walk_data = exec_dust(dust_params);
            println!("Walk done.");

            match walk_data {
                // ノードに要素がある場合
                Some(top_level_nodes) => {
                    println!("Node found.");

                    let encode_result: Result<String, _> = serde_json::to_string(&top_level_nodes);

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
        // パラメータのデコードに失敗した場合
        Err(err) => {
            println!("{}", err.to_string());
            Err(err.to_string())
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            walk_start
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
