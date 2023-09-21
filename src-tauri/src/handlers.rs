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
    dto::response::{
        graph_response::GraphResponse, structure_response::StructureResponse, ResponseError,
        SelectFilesResponse, SelectFolderResponse,
    },
    models::core::structure::{self, Metric, Structure},
    services::{csv_service, structure_service},
    state::AppState,
    transformers::structure_transformer,
};

#[tauri::command]
pub fn get_graph(
    app_state_mutex: State<Mutex<AppState>>,
    graph_id: String,
) -> Result<GraphResponse, ResponseError> {
    let graph_response = graph_controller::get_graph(&app_state_mutex, &graph_id)?;
    Ok(graph_response)
}

#[tauri::command]
pub fn select_files(
    app_state_mutex: State<Mutex<AppState>>,
) -> Result<SelectFilesResponse, ResponseError> {
    let (sender, reciever) = std::sync::mpsc::channel();
    FileDialogBuilder::new().pick_files(move |file_paths| {
        let sender_clone = sender.clone();
        thread::spawn(move || -> Result<()> {
            sender_clone.send(file_paths)?;
            Ok(())
        });
    });

    let mut select_files_res = SelectFilesResponse::builder();

    select_files_res.graph_id("graph id".to_owned());

    if let Ok(Some(path_bufs)) = reciever.recv() {
        graph_controller::create_graph(&app_state_mutex, path_bufs)?;
    }

    Ok(select_files_res.build())
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
