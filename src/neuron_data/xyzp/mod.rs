mod neuron_xyzp;
mod neuron_xyzp_arrays;
mod neuron_xyzp_mappings;
mod coder_traits;

pub mod decoders;
pub mod encoders;

pub use coder_traits::{NeuronXYZPEncoder, NeuronXYZPDecoder};

pub use neuron_xyzp::NeuronXYZP as NeuronXYZP;

pub use neuron_xyzp_arrays::NeuronXYZPArrays as NeuronXYZPArrays;

pub use neuron_xyzp_mappings::CorticalMappedXYZPNeuronData as CorticalMappedXYZPNeuronData;