pub mod neuron_potentials;

pub trait FEAGI_Byte_Structure {
    fn get_ID(&self) -> u8;
    fn get_version(&self) -> u8;
}