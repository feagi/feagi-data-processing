pub mod b001_json;

use crate::error::DataProcessingError;

trait FeagiByteSerializer {
    fn get_id(&self) -> u8;
    fn get_version(&self) -> u8;
    fn get_max_possible_size_when_serialized(&self) -> usize;
    fn serialize_new(&self) -> Result<Vec<u8>, DataProcessingError>;
    fn serialize_in_place(&self, bytes_to_overwrite: &mut [u8]) -> Result<usize, DataProcessingError>;
}