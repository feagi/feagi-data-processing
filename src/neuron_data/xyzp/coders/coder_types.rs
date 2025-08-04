//! Neural coder type variants for different data encoding schemes.
//!
//! This module defines the `NeuronCoderVariantType` enum, which identifies and creates
//! appropriate neural encoders for different types of input data. Each variant corresponds
//! to a specific encoding strategy optimized for particular data characteristics and
//! neural processing requirements.

use crate::error::{FeagiDataProcessingError};
use crate::genomic_structures::{CorticalGroupingIndex, CorticalID, SingleChannelDimensions};
use crate::neuron_data::xyzp::coders::{NeuronXYZPEncoder};
use crate::neuron_data::xyzp::coders::encoders::{ImageFrameNeuronXYZPEncoder, F32PSPBidirectionalNeuronXYZPEncoder, F32SplitSignDividedNeuronXYZPEncoder, F32LinearNeuronXYZPEncoder};

/// Neural encoder type variants for different data encoding schemes.
///
/// This enum identifies the available neural coding strategies and provides factory
/// methods for creating appropriate encoder instances. Each variant is optimized
/// for specific types of input data and neural processing requirements.
///
/// # Encoding Strategies
///
/// ## Linear Encoding
/// - **F32Normalized0To1_Linear**: Direct mapping of normalized [0,1] values
/// - Preserves proportional relationships between input values
/// - Optimal for brightness, probability, or other naturally positive signals
///
/// ## Bidirectional Encoding
/// - **F32NormalizedM1To1_PSPBidirectional**: Post-synaptic potential encoding
/// - **F32NormalizedM1To1_SplitSignDivided**: Separate positive/negative neural populations
/// - Handle control signals requiring both positive and negative values
/// - Suitable for motor control, steering, or directional commands
///
/// ## Visual Encoding
/// - **ImageFrame**: Spatial encoding of 2D image data
/// - **SegmentedImageFrame**: Multi-region encoding for peripheral vision
/// - Maintains spatial topology in neural representation
/// - Optimized for vision processing and spatial data
///
/// # Data Type Compatibility
///
/// Each encoder variant is designed for specific I/O data types:
/// - Linear encoders work with `IOTypeData::F32Normalized0To1`
/// - Bidirectional encoders work with `IOTypeData::F32NormalizedM1To1`
/// - Visual encoders work with `IOTypeData::ImageFrame` and `IOTypeData::SegmentedImageFrame`
///
/// # Performance Characteristics
///
/// - **Linear encoders**: Fastest encoding with minimal computation
/// - **Bidirectional encoders**: Moderate overhead for sign handling
/// - **Visual encoders**: Higher computational cost but maintains spatial relationships
/// - **Segmented encoders**: Most complex, handles multiple spatial regions
pub enum NeuronCoderVariantType { // Enum itself must be exposed (methods don't)
    F32Normalized0To1_Linear,
    F32NormalizedM1To1_PSPBidirectional,
    F32NormalizedM1To1_SplitSignDivided,
    ImageFrame,
    SegmentedImageFrame,
}

impl NeuronCoderVariantType {
    /// Creates a neural encoder instance for single-channel input processing units.
    ///
    /// This factory method instantiates the appropriate encoder implementation based on
    /// the coder variant type. It validates that the cortical area is a sensor type
    /// (input processing unit) and creates an encoder configured for the specified
    /// cortical area and channel dimensions.
    ///
    /// # Arguments
    /// * `cortical_id` - Identifier of the cortical area that will receive encoded data
    /// * `validated_channel_dimensions` - Pre-validated spatial dimensions for the channel
    ///
    /// # Returns
    /// * `Ok(Box<dyn NeuronXYZPEncoder>)` - Configured encoder instance
    /// * `Err(FeagiDataProcessingError)` - If cortical area is not a sensor or encoder creation fails
    ///
    /// # Validation
    /// - Ensures the cortical area is a sensor type (only input areas can have encoders)
    /// - Assumes channel dimensions are already validated against cortical type constraints
    /// - Rejects segmented image frame requests (requires special multi-dimension handling)
    ///
    /// # Encoder Selection
    /// - **Linear**: For normalized [0,1] float values
    /// - **PSP Bidirectional**: For [-1,1] values using post-synaptic potential encoding
    /// - **Split Sign Divided**: For [-1,1] values using separate positive/negative populations
    /// - **Image Frame**: For spatial 2D image data encoding
    /// # Thread Safety
    /// The returned encoder implements `Sync + Send` for safe use in multi-threaded environments.
    pub(crate) fn instantiate_single_ipu_encoder(&self, cortical_id: &CorticalID, validated_channel_dimensions: &SingleChannelDimensions) // Doesn't need to be exposed out of crate
        -> Result<Box<dyn NeuronXYZPEncoder + Sync + Send>, FeagiDataProcessingError> {

        // Assuming channel_dimensions is validated

        if !cortical_id.get_cortical_type().is_type_sensor() {
            return Err(FeagiDataProcessingError::InternalError("Only IPUs can spawn encoders!".into()))
        }

        match self {
            NeuronCoderVariantType::F32Normalized0To1_Linear => {
                Ok(Box::new(F32LinearNeuronXYZPEncoder::new(cortical_id.clone(), validated_channel_dimensions.clone())))
            }
            NeuronCoderVariantType::F32NormalizedM1To1_PSPBidirectional => {
                Ok(Box::new(F32PSPBidirectionalNeuronXYZPEncoder::new(cortical_id.clone(), validated_channel_dimensions.clone())))
            }

            NeuronCoderVariantType::F32NormalizedM1To1_SplitSignDivided => {
                Ok(Box::new(F32SplitSignDividedNeuronXYZPEncoder::new(cortical_id.clone(), validated_channel_dimensions.clone())))
            }

            NeuronCoderVariantType::ImageFrame => {
                Ok(Box::new(ImageFrameNeuronXYZPEncoder::new(cortical_id.clone(), validated_channel_dimensions.clone())))
            }
            NeuronCoderVariantType::SegmentedImageFrame => {
                Err(FeagiDataProcessingError::InternalError("Segmented Image Frame is not a single IPU encoder!".into()))
            }
        }
    }

    // TODO instantiate segmented image frame encoder, find a way to pass in the multiple dimensions
}
