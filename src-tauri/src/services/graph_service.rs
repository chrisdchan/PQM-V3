use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Result};
use tauri::State;
use crate::models::graph::Graph;
use crate::state::AppState;

pub fn get_graph(state: &State<Mutex<AppState>>, graph_id: &str) -> Result<Arc<Graph>> {
    let app_state = state.lock().map_err(|_| anyhow!("Error Accessing State"))?;

    let graph = match app_state.current_graph.as_ref() {
        Some(graph) => Ok(graph),
        None => Err(anyhow!("Graph of id {} does not exist", graph_id)),
    }?;

    if graph.get_id().to_string() == graph_id {
        Ok(Arc::clone(graph))
    } else {
        Err(anyhow!("Graph of id {} does not exist", graph_id))
    }
}
