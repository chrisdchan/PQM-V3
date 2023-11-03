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
use crate::services::graph_service;

pub fn get_graph(state: &State<Mutex<AppState>>, graph_id: &str) -> Result<GraphDisplay> {
    let graph = graph_service::get_graph(state, graph_id)?;
    graph_transformer::to_graph_display(graph.as_ref())
}
pub fn create_graph(
    app_state_mutex: &State<Mutex<AppState>>,
    path_bufs: Vec<PathBuf>,
) -> Result<GraphDisplay> {
    let mut app_state =
        app_state_mutex
        .lock()
        .map_err(|_| anyhow!("Error accessing state"))?;

    let structures: Vec<Structure> = path_bufs
        .into_iter()
        .map(structure_service::create_structure)
        .collect::<Result<Vec<Structure>>>()?;
    let structures_map: HashMap<Uuid, Arc<Structure>> = structures
        .into_iter()
        .map(|structure| (*structure.get_id(), Arc::new(structure)))
        .collect();

    let id = Uuid::new_v4();
    let graph = Graph::new(id,
                           GraphType::CurrentDensity,
                           structures_map,
                           GraphDisplayProperties::default(),
                           GraphDisplayStyle::default());

    let graph_display = graph_transformer::to_graph_display(&graph)?;

    app_state.current_graph = Some(Arc::new(graph));

    Ok(graph_display)
}
