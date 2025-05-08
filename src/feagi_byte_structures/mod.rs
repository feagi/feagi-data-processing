pub mod neuron_potentials;

pub trait FeagiByteStructure {
    fn get_id(&self) -> u8;
    fn get_version(&self) -> u8;
}