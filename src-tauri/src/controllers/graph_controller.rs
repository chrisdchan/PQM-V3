use anyhow::Result;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    dto::{
        input::structure_properties::StructureDisplayProperties,
        response::{
            graph_response::GraphResponse, structure_response::StructureResponse, ResponseError,
        },
    },
    state::AppState,
};

use super::structure_controller;

pub fn create_graph(state: Arc<Mutex<AppState>>, path_bufs: Vec<PathBuf>) -> Result<GraphResponse> {
    let default_display_properties = StructureDisplayProperties::default();
    let structure_responses: Vec<Result<StructureResponse, ResponseError>> = path_bufs
        .into_iter()
        .map(|path_buf| {
            structure_controller::create_structure(
                Arc::clone(&state),
                path_buf,
                &default_display_properties,
            )
        })
        .collect();

    let graph_response = GraphResponse::builder()
        .structures(structure_responses)
        .build();
    Ok(graph_response)
}
