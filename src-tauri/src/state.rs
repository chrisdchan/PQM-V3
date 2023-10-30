use crate::models::graph::Graph;

#[derive(Debug)]
pub struct AppState {
    pub current_graph: Option<Graph>,
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            current_graph: None,
        }
    }
}
