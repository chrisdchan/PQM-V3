use serde::Serialize;

use super::{structure_response::StructureResponse, Builder, ResponseError};

#[derive(Serialize, Clone, Builder)]
pub struct GraphResponse {
    structures: Vec<Result<StructureResponse, ResponseError>>,
}

impl Default for GraphResponse {
    fn default() -> Self {
        Self { structures: vec![] }
    }
}
