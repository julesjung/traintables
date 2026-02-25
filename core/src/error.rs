use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("network error")]
    Network(#[from] reqwest::Error),
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("csv error")]
    Csv(#[from] csv::Error),
    #[error("zip error")]
    Zip(#[from] zip::result::ZipError),
}
