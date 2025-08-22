mod coder_traits;
pub(crate) mod decoders;
pub(crate) mod encoders;

// The only thing that *may* be used outside of this crate is the NeuronEncoderVariantType enum.
// The encoder logic is internal to this crate and nothing needs to know the details of the
// encoders themselves, just their trait (which is spawned by the methods of NeuronEncoderVariantType)

pub(crate) use coder_traits::{NeuronXYZPEncoder};