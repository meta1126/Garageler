// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

use tauri::{window, Window};
use serde::Serialize;
use std::fs::File;
use std::path::Path;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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

fn main() {
    tauri::Builder::default()
    .setup(|app| {
        let id  = app.listen_global("click", |event| {
            println!("got 'click' event with payload{:?}",event.payload());
        });
        Ok(())
    })
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![filesearch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
