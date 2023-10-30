use std::sync::Mutex;
use tauri::State;
use crate::dto::api::StructureTable;
use crate::state::AppState;
use anyhow::{anyhow, Result};

pub fn get_table(
    app_state_mutex: &State<Mutex<AppState>>,
    id: String) -> Result<StructureTable> {
    Err(anyhow!("Unimplemented"))
}