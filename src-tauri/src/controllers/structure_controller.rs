use std::sync::Mutex;
use tauri::State;
use crate::dto::api::StructureTable;
use crate::state::AppState;
use anyhow::{anyhow, Result};
use crate::services::structure_service;
use crate::transformers::structure_transformer;

pub fn get_table(
    app_state: &State<Mutex<AppState>>,
    graph_id: String,
    id: String) -> Result<StructureTable> {

    let structure = structure_service::get_structure(app_state, &graph_id, &id)?;
    structure_transformer::to_structure_table(&structure)
}