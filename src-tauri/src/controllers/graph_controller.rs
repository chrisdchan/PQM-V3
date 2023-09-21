use anyhow::{anyhow, Result};
use std::{
    path::{self, Path, PathBuf},
    sync::{Arc, Mutex},
};
use tauri::State;

use crate::{
    dto::response::{
        graph_response::GraphResponse, structure_response::StructureResponse, ResponseError,
    },
    models::core::{graph::Graph, structure::Structure},
    services::structure_service::{self, create_structure},
    state::AppState,
    transformers::structure_transformer,
};

pub fn get_graph(state: &State<Mutex<AppState>>, graph_id: &str) -> Result<GraphResponse> {
    let app_state = state.lock().map_err(|_| anyhow!("Error Accessing State"))?;

    match &app_state.current_graph {
        Some(graph) => {
            let structure_responses: Vec<StructureResponse> = graph
                .get_structures()
                .into_iter()
                .map(|structure| structure_transformer::to_structure_response(structure))
                .collect::<Result<Vec<StructureResponse>>>()?;

            let graph_response = GraphResponse::builder()
                .structures(structure_responses)
                .build();
            Ok(graph_response)
        }
        None => Err(anyhow!("Graph of id {} does not exist", graph_id)),
    }
}

pub fn create_graph(
    app_state_mutex: &State<Mutex<AppState>>,
    path_bufs: Vec<PathBuf>,
) -> Result<GraphResponse> {
    let mut app_state = app_state_mutex.lock().map_err(|_| anyhow!("Error accessing state"))?;

    let structures: Vec<Structure> = path_bufs
        .into_iter()
        .map(structure_service::create_structure)
        .collect::<Result<Vec<Structure>>>()?;

    let structure_responses: Vec<StructureResponse> = structures
        .clone()
        .into_iter()
        .map(|structure| structure_transformer::to_structure_response(&structure))
        .collect::<Result<Vec<StructureResponse>>>()?;

    let graph_response = GraphResponse::builder()
        .structures(structure_responses)
        .build();

    let graph = Graph::new(structures);
    app_state.current_graph = Some(graph);

    Ok(graph_response)
}
