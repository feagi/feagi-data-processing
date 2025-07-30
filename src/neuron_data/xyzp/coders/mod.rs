mod coder_traits;
mod coder_types;
mod decoders;
mod encoders;

// The only thing that *may* be used outside of this crate is the NeuronEncoderVariantType enum.
// The encoder logic is internal to this crate and nothing needs to know the details of the
// encoders themselves, just their trait (which is spawned by the methods of NeuronEncoderVariantType)

pub use coder_types::{NeuronCoderVariantType};
pub(crate) use coder_traits::{NeuronXYZPEncoder, NeuronXYZPDecoder};