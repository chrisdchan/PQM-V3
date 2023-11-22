use anyhow::{anyhow, Error, Result};
use std::{any, path::PathBuf, vec};
use std::sync::Arc;
use csv::Writer;

use crate::{dto::structure_dto::StructureDto, models::structure::Metric};
use crate::dto::api::GraphTableDisplay;
use crate::models::structure::Structure;

pub fn read_csv(file_path: &PathBuf) -> Result<String> {
    println!("Processing {:?}", file_path);
    match std::fs::read_to_string(file_path) {
        Ok(csv_string) => Ok(csv_string),
        Err(e) => Err(anyhow!("Unable to read file")),
    }
}

pub fn write_graph_table_to_csv(graph_table: GraphTableDisplay, file_path: String) -> Result<()> {
    let mut wtr = Writer::from_path(file_path)?;
    wtr.write_record(&["Hello"])?;
    print!("Write CSV File");
    Ok(())
}