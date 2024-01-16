use std::sync::Arc;
use crate::dto::api::{GraphDisplayProperties, GraphDisplayStyle};
use crate::models::graph::Graph;

#[derive(Debug)]
pub struct AppState {
    pub current_graph: Option<Arc<Graph>>,
    pub graph_display_properties: Option<Arc<GraphDisplayProperties>>,
    pub graph_display_style: Option<Arc<GraphDisplayStyle>>,
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            current_graph: None,
            graph_display_properties: None,
            graph_display_style: None
        }
    }
}
