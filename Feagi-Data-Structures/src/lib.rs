pub mod basic_components;
pub mod bytes;
mod templates;
mod error;
pub mod genomic;
pub mod neurons;
mod data;
mod io_containers;
mod processing;

pub use templates::*;
pub use error::FeagiDataError as FeagiDataError;
