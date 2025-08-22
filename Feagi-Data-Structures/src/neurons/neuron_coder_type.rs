/// Internal Enum used to determine what type of encoder / decoder a sensor / motor type gets. Essentially only used
/// by macros

/// Neural encoder type variants for different data encoding schemes.
pub enum NeuronCoderType {
    #[allow(non_camel_case_types)] F32Normalized0To1_Linear, // Due to the segmented nature, do this
    #[allow(non_camel_case_types)] F32NormalizedM1To1_PSPBidirectional,
    #[allow(non_camel_case_types)] F32NormalizedM1To1_SplitSignDivided,
    ImageFrame,
    SegmentedImageFrame,
}