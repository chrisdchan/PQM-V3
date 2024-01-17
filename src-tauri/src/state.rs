use std::sync::{Arc, Mutex};

use crate::models::graph::Graph;
use crate::services::csv_service::CsvService;
use crate::services::implementations::csv_service_impl::CsvServiceImpl;

#[derive(Debug)]
pub struct AppState {
    pub current_graph: Arc<Mutex<Option<Graph>>>,
    pub csv_service: Arc<dyn CsvService>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_graph: None,
            csv_service: Arc::new(CsvServiceImpl {}),
        }
    }
}
