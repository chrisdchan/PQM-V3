use std::path::PathBuf;
use anyhow::{Result, anyhow};

pub fn to_file_name(path_buf: &PathBuf) -> Result<String> {
    let path_string = format!("{:?}", path_buf);
    let file_name = path_string.split("\\").last().ok_or(anyhow!("Could not extract file name form {}", path_string))?;
    Ok(file_name.to_owned())
}
