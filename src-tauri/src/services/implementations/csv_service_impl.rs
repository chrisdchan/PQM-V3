use std::path::PathBuf;

use anyhow::{anyhow, Result};
use csv::Writer;

use crate::dto::api::GraphTableDisplay;
use crate::services::csv_service::CsvService;

#[derive(Debug)]
pub struct CsvServiceImpl {}

impl CsvService for CsvServiceImpl {
    fn read_csv(&self, file_path: &PathBuf) -> Result<String> {
        println!("Processing {:?}", file_path);
        match std::fs::read_to_string(file_path) {
            Ok(csv_string) => Ok(csv_string),
            Err(e) => Err(anyhow!("Unable to read file")),
        }
    }
    fn write_graph_table_to_csv(
        &self,
        graph_table: GraphTableDisplay,
        file_path: String,
    ) -> Result<()> {
        let mut wtr = Writer::from_path(file_path)?;
        wtr.write_record(&["Hello"])?;
        print!("Write CSV File");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn needs_send<T: Send>(_: T) {}

    #[test]
    fn implements_send() {
        let csv_service = CsvServiceImpl {};
        needs_send(csv_service);
    }
}
