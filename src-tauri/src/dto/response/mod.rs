use lombok::Builder;
use serde::Serialize;
use std::sync::{mpsc::RecvError, PoisonError};

use crate::models::core::graph::Graph;

use self::graph_response::GraphResponse;

pub mod curve;
pub mod graph_response;
pub mod structure_response;

#[derive(Serialize, Debug, thiserror::Error, Clone)]
#[error("Response Error")]
pub struct ResponseError {
    msg: String,
}

impl From<anyhow::Error> for ResponseError {
    fn from(value: anyhow::Error) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl<T> From<PoisonError<T>> for ResponseError {
    fn from(value: PoisonError<T>) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl From<RecvError> for ResponseError {
    fn from(value: RecvError) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

#[derive(Serialize, Builder)]
pub struct SelectFilesResponse {
    graph_response: GraphResponse,
}

#[derive(Serialize, Builder)]
pub struct SelectFolderResponse {
    graph_response: GraphResponse,
}
