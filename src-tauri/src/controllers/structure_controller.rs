use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    dto::{
        input::structure_properties::StructureDisplayProperties,
        response::{structure_response::StructureResponse, ResponseError},
    },
    services::structure_service,
    state::AppState,
    transformers::structure_transformer,
};

pub fn create_structure(
    state: Arc<Mutex<AppState>>,
    path_buf: PathBuf,
    display_properties: &StructureDisplayProperties,
) -> Result<StructureResponse, ResponseError> {
    let structure = structure_service::create_structure(path_buf)?;
    let structure_response =
        structure_transformer::to_structure_response(&structure, display_properties)?;
    Ok(structure_response)
}
