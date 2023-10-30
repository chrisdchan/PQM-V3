// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_imports, unused_variables)]

use std::sync::Mutex;

use state::AppState;
use tauri::generate_handler;

use crate::handlers::{get_graph, select_files};

pub mod controllers;
pub mod dto;
pub mod handlers;
pub mod models;
pub mod services;
pub mod state;
pub mod transformers;
pub mod utils;
fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(generate_handler![select_files, get_graph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
