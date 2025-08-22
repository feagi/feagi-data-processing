pub mod basic_components;
pub mod bytes;
pub mod data;
pub mod wrapped_io_data;
pub mod genomic;
pub mod neurons;
mod templates;
mod error;


mod processing;

pub use templates::*;
pub use error::FeagiDataError as FeagiDataError;
