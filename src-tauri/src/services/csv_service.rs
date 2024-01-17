use std::fmt::Debug;
use std::path::PathBuf;

use anyhow::Result;

use crate::dto::api::GraphTableDisplay;

pub trait CsvService: Send + Sync + Debug {
    fn read_csv(&self, file_path: &PathBuf) -> Result<String>;
    fn write_graph_table_to_csv(
        &self,
        graph_table: GraphTableDisplay,
        file_path: String,
    ) -> Result<()>;
}
