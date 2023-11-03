use std::sync::Mutex;
use tauri::State;
use crate::dto::api::StructureTable;
use crate::state::AppState;
use anyhow::{anyhow, Result};

pub fn get_table(
    app_state_mutex: &State<Mutex<AppState>>,
    graph_id: String,
    id: String) -> Result<StructureTable> {
    let mut app_state =
        app_state_mutex
            .lock()
            .map_err(|_| anyhow!("Error accessing state"))?;

    Err(anyhow!("unimplemented"))
}