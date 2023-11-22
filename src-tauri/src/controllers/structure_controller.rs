use std::sync::Mutex;
use tauri::State;
use crate::state::AppState;
use anyhow::{anyhow, Result};
use crate::services::structure_service;
use crate::transformers::structure_transformer;
