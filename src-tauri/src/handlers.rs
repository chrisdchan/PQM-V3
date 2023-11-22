use anyhow::{anyhow, bail, Result};
use std::{
    cell::{RefCell, RefMut},
    path::{self, Path, PathBuf},
    rc::Rc,
    sync::{mpsc::RecvError, Arc, Mutex, PoisonError},
    thread,
};
use tauri::{
    api::dialog::{self, FileDialogBuilder},
    State,
};

use crate::{
    controllers::graph_controller,
    state::AppState,
    transformers::structure_transformer, dto::{api::GraphDisplay, api_error::ResponseError},
};
use crate::controllers::structure_controller;
use crate::dto::api::{GraphTableDisplay};

#[tauri::command]
pub fn get_graph(
    app_state_mutex: State<Mutex<AppState>>,
    graph_id: String,
) -> Result<GraphDisplay, ResponseError> {
    let graph_response = graph_controller::get_graph(&app_state_mutex, &graph_id)?;
    Ok(graph_response)
}

#[tauri::command]
pub fn select_files(
    app_state_mutex: State<Mutex<AppState>>,
) -> Result<GraphDisplay, ResponseError> {
    let (sender, reciever) = std::sync::mpsc::channel();
    FileDialogBuilder::new().pick_files(move |file_paths| {
        let sender_clone = sender.clone();
        thread::spawn(move || -> Result<()> {
            sender_clone.send(file_paths)?;
            Ok(())
        });
    });

    return if let Ok(Some(path_bufs)) = reciever.recv() {
        let graph_display = graph_controller::create_graph(&app_state_mutex, path_bufs)?;
        Ok(graph_display)
    } else {
        Err(ResponseError::new("Unable to detect file selection".to_string()))
    } 
}

// #[tauri::command]
// pub fn select_folder() -> Result<SelectFolderResponse, ResponseError> {
//     let (sender, reciever) = std::sync::mpsc::channel();
//     FileDialogBuilder::new().pick_folder(move |folder_path| {
//         let sender_clone = sender.clone();
//         thread::spawn(move || -> Result<()> {
//             sender_clone.send(folder_path)?;
//             Ok(())
//         });
//     });

//     let mut graph_response = GraphResponse::default();

//     if let Some(path_bufs) = reciever.recv()? {
//         graph_response = graph_controller::create_graph(Arc::clone(&state), path_bufs)?;
//     }

//     let select_files_res = SelectFilesResponse::builder()
//         .graph_response(graph_response)
//         .build();
//     Ok(select_files_res)
// }
#[tauri::command]
pub fn get_graph_table(
    app_state: State<Mutex<AppState>>,
    graph_id: String) -> Result<GraphTableDisplay, ResponseError> {
    match graph_controller::get_graph_table(&app_state, &graph_id) {
        Ok(graphTableDisplay) => Ok(graphTableDisplay),
        Err(e) => Err(ResponseError::new(e.to_string()))
    }
}
#[tauri::command]
pub fn export_graph_table(
    app_state: State<Mutex<AppState>>,
    graph_id: String,
    path: String
) {
    println!("This will eventually save Graph to {}", path)
}