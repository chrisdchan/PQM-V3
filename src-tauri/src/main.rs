// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_imports, unused_variables)]
use crate::handlers::{export_graph_table, get_graph, get_graph_table, select_files};
use crate::services::implementations::csv_service_impl::CsvServiceImpl;
use state::AppState;
use std::sync::{Arc, Mutex};
use tauri::generate_handler;

#[macro_use]
extern crate public;

pub mod controllers;
pub mod dto;
mod errors;
pub mod handlers;
pub mod models;
pub mod services;
pub mod state;
pub mod transformers;
pub mod utils;

fn main() {
    let csv_service = CsvServiceImpl {};

    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(generate_handler![
            select_files,
            get_graph,
            get_graph_table,
            export_graph_table
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
