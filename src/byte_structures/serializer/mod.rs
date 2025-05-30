pub mod b001_json;
mod b011_neuron_categorical_xyzp;
mod b009_multi_struct_holder;

use crate::cortical_data::CorticalID;
use crate::error::DataProcessingError;

trait FeagiByteSerializer {
    fn get_id(&self) -> u8;
    fn get_version(&self) -> u8;
    fn get_max_possible_size_when_serialized(&self) -> usize;
    fn serialize_new(&self) -> Result<Vec<u8>, DataProcessingError>;
    fn serialize_in_place(&self, bytes_to_overwrite: &mut [u8]) -> Result<usize, DataProcessingError>;
    fn generate_global_header(&self) ->[u8; 2] {
        [self.get_id(), self.get_version()]
    }
}