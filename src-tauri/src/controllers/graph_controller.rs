use std::collections::HashMap;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use tauri::State;
use uuid::Uuid;

use crate::dto::api::{GraphTableDisplay, StructureTableDisplay};
use crate::services::graph_service;
use crate::{
    dto::api::GraphDisplay,
    models::{graph::Graph, structure::Structure},
    services::structure_service::create_structure,
    state::AppState,
    transformers::{graph_transformer, structure_transformer},
};

pub fn get_graph(state: State<AppState>, graph_id: &str) -> Result<GraphDisplay> {
    let graph_mutex = graph_service::get_graph(state, graph_id)?;
    let graph = graph_mutex.lock().unwrap();
    graph_transformer::to_graph_display(&graph)
}

pub fn create_graph(state: State<AppState>, path_bufs: Vec<PathBuf>) -> Result<GraphDisplay> {
    let structures: Vec<Structure> = path_bufs
        .into_iter()
        .map(create_structure)
        .collect::<Result<Vec<Structure>>>()?;
    let structures_map: HashMap<Uuid, Arc<Mutex<Structure>>> = structures
        .into_iter()
        .map(|structure| (*structure.get_id(), Arc::new(Mutex::new(structure))))
        .collect();

    let id = Uuid::new_v4();
    let graph = Graph::builder()
        .id(id)
        .graph_type(None)
        .structures(structures_map)
        .build();

    let graph_display = graph_transformer::to_graph_display(&graph)?;

    let mut current_graph = state.current_graph.lock().unwrap();
    current_graph.replace(Arc::new(Mutex::new(graph)));
    Ok(graph_display)
}

pub fn get_graph_table(state: State<AppState>, graph_id: &str) -> Result<GraphTableDisplay> {
    let graph = graph_service::get_graph(state, graph_id)?;
    let graph = graph.lock().unwrap();
    let structure_table_displays: Vec<StructureTableDisplay> = graph
        .get_structures()
        .values()
        .into_iter()
        .map(|structure| structure_transformer::to_structure_table(Arc::clone(&structure)))
        .collect::<Result<Vec<StructureTableDisplay>>>()?;
    let graph_table_display = GraphTableDisplay::new(structure_table_displays);
    Ok(graph_table_display)
}
