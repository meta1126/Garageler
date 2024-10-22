// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use serde_json::error;
use tauri::http::header::Entry;
use tauri::Manager;
use tokio::task;
use walkdir::{DirEntry, WalkDir};
use thiserror::Error;
use log::{error, info, warn};
use env_logger;



use tauri::{window, Window};
use serde::Serialize;
use std::fmt::format;
use std::fs::File;
use std::path::Path;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(Error, Debug)]
enum AppError {
    #[error("Failed to list files: {0}")]
    FileListError(String),
}
#[derive(Serialize, Debug)]
struct  FileList{
    directories: Vec<String>,
    files: Vec<String>,
}

#[tauri::command]
fn filesearch(fname: &str) -> String{
    let file_path = Path::new(fname);
    if file_path.exists(){
        format!("ファイルがございます。{}", fname)
    }else{
        format!("ファイルがNAI！{}",fname)
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
    .to_str()
    .map(|s| s.starts_with("."))
    .unwrap_or(false)
}

#[tauri::command]
async fn list_files_and_dirs(window: tauri::Window, path: String, 
max_entries: usize) -> Result<FileList, String> {
  task::spawn_blocking(move ||{
    info!("ディレクトリ走査開始: {}", &path);
    let dir_path = Path::new(&path);
    let mut files = Vec::with_capacity(max_entries / 1);
    let mut directories = Vec::with_capacity(max_entries / 1);

    if !dir_path.exists() {
        let error_msg = format!("指定されたディレクトリが存在しません: {}", path);
        window.emit("error", &error_msg).unwrap();
        return  Err( error_msg);
    }

    for (index, entry) in WalkDir::new(path)
    .max_depth(1)
    .contents_first(false)
    .into_iter()
    .filter_entry(|e| !is_hidden(e))
    .take(max_entries)
    .enumerate()
    {
        if index %1000 == 0 {
            window.emit("progress", index).unwrap();
        }
        match entry{
            Ok(entry) => {
                if entry.file_type().is_file(){
                    files.push(entry.path().display().to_string());
                } else if entry.file_type().is_dir() {
                    directories.push(entry.path().display().to_string());
                }
            }
            Err(e)=> {
                let error_msg = format!("エラーが発生しました: {}", e);
                window.emit("error", &error_msg).unwrap();
                warn!("エラーが発生しました: {}", e);
            }
        }
    }
    Ok(FileList { directories, files})
}).await.unwrap()
}


#[tokio::main]
 async fn main() {
    env_logger::init();
    tauri::Builder::default()
        .setup(|app| {
                let main_window = app.get_window("main").unwrap();
                let window_clone = main_window.clone();
                main_window.listen("error",move |event| {
                    let err_message = event.payload().unwrap();
                    window_clone.emit("display_error", format!("エラーが発生しました: {}", err_message)).unwrap();
                });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![filesearch, list_files_and_dirs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
