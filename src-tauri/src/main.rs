// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::generate_handler;

use crate::handlers::select_files;

pub mod controllers;
pub mod dto;
pub mod models;
pub mod services;
pub mod transformers;
pub mod utils;
pub mod handlers;
pub mod state;


fn main() {
  tauri::Builder::default()
  .invoke_handler(generate_handler![select_files])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
