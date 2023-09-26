use std::sync::{PoisonError, mpsc::RecvError};

use lombok::AllArgsConstructor;
use serde::Serialize;

#[derive(Serialize, Debug, thiserror::Error, Clone, AllArgsConstructor)]
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