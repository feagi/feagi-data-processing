// Module for image data structures for FEAGI. Essentially pixel data stored uncompressed in ndarrays

use ndarray::{s, Array3, ArrayView3};
use crate::error::DataProcessingError;
use crate::neuron_data::NeuronXYCPArrays;
use descriptors::*;

pub mod segmented_vision_frame;
pub mod descriptors;
pub mod image_frame;
pub mod quick_image_diff;


