use anyhow::{anyhow, Result};
use uuid::Uuid;
use std::{
    path::{self, Path, PathBuf},
    sync::{Arc, Mutex},
};
use std::collections::HashMap;
use tauri::State;

use crate::{
    models::{graph::Graph, structure::Structure},
    services::structure_service::{self, create_structure},
    state::AppState,
    transformers::{structure_transformer, graph_transformer}, dto::api::{GraphDisplay, StructureDisplay, GraphDisplayProperties, GraphDisplayStyle},
};
use crate::dto::api::GraphType;
pub fn get_graph(state: &State<Mutex<AppState>>, graph_id: &str) -> Result<GraphDisplay> {
    let app_state = state.lock().map_err(|_| anyhow!("Error Accessing State"))?;

    match &app_state.current_graph {
        Some(graph) => graph_transformer::to_graph_display(graph),
        None => Err(anyhow!("Graph of id {} does not exist", graph_id)),
    }
}
pub fn create_graph(
    app_state_mutex: &State<Mutex<AppState>>,
    path_bufs: Vec<PathBuf>,
) -> Result<GraphDisplay> {
    let mut app_state = app_state_mutex.lock().map_err(|_| anyhow!("Error accessing state"))?;

    let structures: Vec<Structure> = path_bufs
        .into_iter()
        .map(structure_service::create_structure)
        .collect::<Result<Vec<Structure>>>()?;
    let structures_map: HashMap<Uuid, Structure> = structures
        .into_iter()
        .map(|structure| (*structure.get_id(), structure))
        .collect();

    let id = Uuid::new_v4();
    let graph = Graph::new(id,
                           GraphType::CurrentDensity,
                           structures_map,
                           GraphDisplayProperties::default(),
                           GraphDisplayStyle::default());

    let graph_display = graph_transformer::to_graph_display(&graph)?;

    app_state.current_graph = Some(graph);

    Ok(graph_display)
}
