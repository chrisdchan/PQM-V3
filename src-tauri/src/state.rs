use std::sync::Arc;
use crate::models::graph::Graph;

#[derive(Debug)]
pub struct AppState {
    pub current_graph: Option<Arc<Graph>>,
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            current_graph: None,
        }
    }
}
