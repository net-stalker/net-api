use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DecodeInsertEntityError {
    #[error("can not decode data into insert entity")]
    WrongInsertRequest,
}
