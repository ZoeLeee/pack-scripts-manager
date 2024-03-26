// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_files(dir: &str) -> Vec<String> {
    // 获取目录的文件信息
    let entries = fs::read_dir(dir).unwrap();

    // 声明一个可变的数组
    let mut files: Vec<String> = Vec::new();

    // 遍历目录中的文件
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        files.push(file_name);
    }

    return files;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
