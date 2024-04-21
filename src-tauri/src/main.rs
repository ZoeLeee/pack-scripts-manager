// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::Value;
use std::fs;
use std::process::Command;
use std::process::Stdio;

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
async fn run_script(path: &str, script: &str) -> Result<String, String> {
    use tokio::process::Command;

    let output = Command::new("npm")
        .args(["run", script])
        .current_dir(path)
        .spawn()
        .map_err(|e| e.to_string())?;

    // if output.status.success() {
    //     let s = String::from_utf8_lossy(&output.stdout);

    //     print!("rustc succeeded and stdout was:\n{}", s);

    //     String::from("success")
    // } else {
    //     let s = String::from_utf8_lossy(&output.stderr);

    //     print!("rustc failed and stderr was:\n{}", s);

    //     String::from("fail")
    // }

    // 等待命令执行完成并捕获输出
    let output = output.wait_with_output().await.map_err(|e| e.to_string())?;

    if output.status.success() {
        // 如果命令执行成功，将输出转换为字符串并返回
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(output_str)
    } else {
        // 如果命令执行失败，返回错误信息
        Err(String::from("Command execution failed"))
    }
}

fn read_package_json(dir: &str) -> serde_json::Result<Value> {
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

    //读取package.json文件
    let package_json_file = package_json_file.unwrap();
    let package_json = fs::read_to_string(package_json_file).unwrap();

    // 解析JSON字符串
    let json: Value = serde_json::from_str(&package_json)?;

    Ok(json)
}

#[tauri::command]
fn get_package_json(dir: &str) -> Result<Value, String> {
    read_package_json(dir).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_files,
            run_script,
            get_package_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
