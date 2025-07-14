mod neuron_xyzp;
mod neuron_xyzp_arrays;
mod cortical_mapped_xyzp_neuron_data;
mod coder_traits;
mod decoders; 
mod encoders;

pub use coder_traits::{NeuronXYZPDecoder, NeuronXYZPEncoder};

pub use neuron_xyzp::NeuronXYZP as NeuronXYZP;

pub use neuron_xyzp_arrays::NeuronXYZPArrays as NeuronXYZPArrays;

pub use cortical_mapped_xyzp_neuron_data::CorticalMappedXYZPNeuronData as CorticalMappedXYZPNeuronData;