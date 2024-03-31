// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_files(dir: &str) -> String {
    // 获取目录的文件信息
    let entries = fs::read_dir(dir).unwrap();

    // 找到目录中的package.json文件
    let mut package_json_file = None;
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.file_name().unwrap() == "package.json" {
            package_json_file = Some(path);
            break;
        }
    }

    // 如果找不到package.json文件，返回空
    if package_json_file.is_none() {
        return String::from("");
    }

    //读取package.json文件
    let package_json_file = package_json_file.unwrap();
    let package_json = fs::read_to_string(package_json_file).unwrap();

    package_json
}

#[tauri::command]
fn run_script(path: &str, script: &str) -> String {
    print!("cmd: {}", script);
    print!("path: {}", path);

    let output = Command::new("npm")
        .arg("run")
        .arg(script)
        .current_dir(path)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);

        String::from("success")
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);

        String::from("fail")
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_files, run_script])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
