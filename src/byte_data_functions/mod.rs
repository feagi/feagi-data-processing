mod byte_structures;
pub mod deserializers;

use std::cmp::PartialEq;

pub const GLOBAL_HEADER_SIZE: usize = 2;

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum FeagiByteStructureType {
    JSON = 1,
    MultiStructHolder = 9,
    NeuronCategoricalXYZP = 11
}



