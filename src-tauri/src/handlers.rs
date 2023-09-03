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
use tokio::sync::oneshot;

use crate::{
    controllers::graph_controller,
    dto::response::{
        graph_response::GraphResponse, ResponseError, SelectFilesResponse, SelectFolderResponse,
    },
    models::core::structure::{self, Metric, Structure},
    services::{csv_service, structure_service},
    state::AppState,
};

#[tauri::command]
pub fn select_files() -> Result<SelectFilesResponse, ResponseError> {
    let mut state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::default()));
    let (sender, reciever) = std::sync::mpsc::channel();
    FileDialogBuilder::new().pick_files(move |file_paths| {
        let sender_clone = sender.clone();
        thread::spawn(move || -> Result<()> {
            sender_clone.send(file_paths)?;
            Ok(())
        });
    });

    let mut graph_response = GraphResponse::default();

    if let Some(path_bufs) = reciever.recv()? {
        graph_response = graph_controller::create_graph(Arc::clone(&state), path_bufs)?;
    }

    let select_files_res = SelectFilesResponse::builder()
        .graph_response(graph_response)
        .build();
    Ok(select_files_res)
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
