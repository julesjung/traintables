use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum GTFSError {
    #[error("network error")]
    Network(#[from] reqwest::Error),
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("csv error")]
    Csv(#[from] csv::Error),
    #[error("zip error")]
    Zip(#[from] zip::result::ZipError),
}
