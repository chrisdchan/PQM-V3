use anyhow::{anyhow, Error, Result};
use std::{any, path::PathBuf, vec};

use crate::{dto::structure_dto::StructureDto, models::core::structure::Metric};

pub fn read_csv(file_path: &PathBuf) -> Result<String> {
    println!("Processing {:?}", file_path);
    match std::fs::read_to_string(file_path) {
        Ok(csv_string) => Ok(csv_string),
        Err(e) => Err(anyhow!("Unable to read file")),
    }
}
