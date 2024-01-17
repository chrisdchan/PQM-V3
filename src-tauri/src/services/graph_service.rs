use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use tauri::State;

use crate::models::graph::Graph;
use crate::state::AppState;

pub fn get_graph(state: State<AppState>, graph_id: &str) -> Result<Arc<Mutex<Graph>>> {
    let graph = state
        .current_graph
        .ok_or_else(|| anyhow!("Graph not found in app state"))?;
    validate_graph_id(Arc::clone(&graph), graph_id)?;
    Ok(Arc::clone(&graph))
}

fn validate_graph_id(graph: Arc<Mutex<Graph>>, graph_id: &str) -> Result<()> {
    let graph = graph.lock().unwrap();
    if graph.get_id().to_string() == graph_id {
        Ok(())
    } else {
        Err(anyhow!(
            "Graph found in app state but has mismatching id:\n \
            expected: {}\n \
            found: {}",
            graph_id,
            graph.get_id().to_string()
        ))
    }
}
