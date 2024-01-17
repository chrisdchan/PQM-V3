use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Error, Result};
use tauri::CursorIcon::Grab;
use tauri::State;

use crate::errors::graph_not_found::GraphNotFoundError;
use crate::models::graph::Graph;
use crate::state::AppState;

/// Extracts graph from state. Validates the id matches
pub fn get_graph(state: State<AppState>, graph_id: &str) -> Result<Arc<Mutex<Graph>>> {
    let current_graph_mutex = state.current_graph.lock().unwrap();
    let graph_mutex = current_graph_mutex
        .as_ref()
        .ok_or_else(|| GraphNotFoundError {})?;

    let graph = graph_mutex.lock().unwrap();

    if matches_id(&graph, graph_id) {
        Ok(Arc::clone(&graph_mutex))
    } else {
        // For now, return an error. In the future, this branch will be a database call
        Err(anyhow!(
            "Graph found in app state but has mismatching id:\n \
            expected: {}\n \
            found: {}",
            graph_id,
            graph.get_id().to_string()
        ))
    }
}

fn matches_id(graph: &Graph, graph_id: &str) -> bool {
    graph.get_id().to_string() == graph_id
}
