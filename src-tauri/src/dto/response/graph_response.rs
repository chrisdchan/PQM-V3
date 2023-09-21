use serde::Serialize;

use crate::models::styles::graph_display_properties::GraphDisplayProperties;

use super::{structure_response::StructureResponse, Builder};

#[derive(Serialize, Clone, Builder)]
pub struct GraphResponse {
    structures: Vec<StructureResponse>,
    graph_properties: GraphDisplayProperties,
}

impl Default for GraphResponse {
    fn default() -> Self {
        Self {
            structures: vec![],
            graph_properties: GraphDisplayProperties::default(),
        }
    }
}
