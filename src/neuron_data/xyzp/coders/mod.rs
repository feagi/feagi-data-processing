mod coder_traits;
mod coder_types;
pub mod decoders; // passes through all the types directly
pub mod encoders;

pub use coder_types::{instantiate_encoder_by_type, NeuronCoderVariantType};
pub use coder_traits::{NeuronXYZPEncoder, NeuronXYZPDecoder};