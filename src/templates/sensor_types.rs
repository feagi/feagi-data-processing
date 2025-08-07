/// Definition File for Sensors (Input Processing Units)
#[macro_export]
macro_rules! sensor_definition {
    ($callback:ident) => {
        $callback! {
            SensorCorticalType {
                #[doc = "Infrared distance sensor for object detection"]
                Infrared => {
                    friendly_name: "Infrared Sensor",
                    base_ascii: b"iinf00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..2, 1..2, 1..2),
                    default_coder_type: NeuronCoderVariantType::F32Normalized0To1_Linear,
                },
                #[doc = "Inverted infrared sensor that provides reverse object detection readings."]
                ReverseInfrared => {
                    friendly_name: "Reverse Infrared Sensor",
                    base_ascii: b"iiif00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..2, 1..2, 1..2),
                    default_coder_type: NeuronCoderVariantType::F32Normalized0To1_Linear,
                },
                #[doc = "Digital GPIO input pin for reading binary signals (high/low states)."]
                DigitalGPIOInput => {
                    friendly_name: "GPIO Digital Input",
                    base_ascii: b"idgp00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..2, 1..2, 1..2),
                    default_coder_type: NeuronCoderVariantType::F32Normalized0To1_Linear,
                },
                #[doc = "Proximity sensor for detecting nearby objects and measuring distances."]
                Proximity => {
                    friendly_name: "Proximity",
                    base_ascii: b"ipro00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..2, 1..2, 1..u32::MAX),
                    default_coder_type: NeuronCoderVariantType::F32Normalized0To1_Linear,
                },
                #[doc = "Shock sensor for sensing 'pain'"]
                Shock => {
                    friendly_name: "Shock",
                    base_ascii: b"ishk00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..2, 1..2, 1..2),
                    default_coder_type: NeuronCoderVariantType::F32Normalized0To1_Linear,
                },
                #[doc = "Battery level sensor for monitoring power remaining."]
                Battery => {
                    friendly_name: "Battery Gauge",
                    base_ascii: b"ibat00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..2, 1..2, 1..u32::MAX),
                    default_coder_type: NeuronCoderVariantType::F32Normalized0To1_Linear,
                },
                #[doc = "Servo position feedback sensor for monitoring actuator positions."]
                ServoPosition => {
                    friendly_name: "Servo Position",
                    base_ascii: b"isvp00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..2, 1..2, 1..u32::MAX),
                    default_coder_type: NeuronCoderVariantType::F32NormalizedM1To1_SplitSignDivided,
                },
                #[doc = "Grayscale vision sensor positioned at the center of the visual field for primary vision processing."]
                VisionCenterGray => {
                    friendly_name: "Center Vision Input (Grayscale)",
                    base_ascii: b"ivcc00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..2),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Grayscale vision sensor positioned at the top left of peripheral vision processing."]
                VisionTopLeftGray => {
                    friendly_name: "Top Left Vision Input (Grayscale)",
                    base_ascii: b"ivtl00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..2),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionTopMiddleGray => {
                    friendly_name: "Top Middle Vision Input (Grayscale)",
                    base_ascii: b"ivtm00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..2),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionTopRightGray => {
                    friendly_name: "Top Right Vision Input (Grayscale)",
                    base_ascii: b"ivtr00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..2),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionMiddleLeftGray => {
                    friendly_name: "Middle Left Vision Input (Grayscale)",
                    base_ascii: b"ivml00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..2),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionMiddleRightGray => {
                    friendly_name: "Middle Right Vision Input (Grayscale)",
                    base_ascii: b"ivmr00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..2),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionBottomLeftGray => {
                    friendly_name: "Bottom Left Vision Input (Grayscale)",
                    base_ascii: b"ivbl00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..2),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionBottomMiddleGray => {
                    friendly_name: "Bottom Middle Vision Input (Grayscale)",
                    base_ascii: b"ivbm00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..2),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionBottomRightGray => {
                    friendly_name: "Bottom Right Vision Input (Grayscale)",
                    base_ascii: b"ivbr00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..2),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Color vision sensor positioned at the center of the visual field for full-spectrum image processing with RGB channels."]
                VisionCenterColor => {
                    friendly_name: "Center Vision Input (Color)",
                    base_ascii: b"iVcc00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionTopLeftColor => {
                    friendly_name: "Top Left Vision Input (Color)",
                    base_ascii: b"iVtl00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionTopMiddleColor => {
                    friendly_name: "Top Middle Vision Input (Color)",
                    base_ascii: b"iVtm00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionTopRightColor => {
                    friendly_name: "Top Right Vision Input (Color)",
                    base_ascii: b"iVtr00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionMiddleLeftColor => {
                    friendly_name: "Middle Left Vision Input (Color)",
                    base_ascii: b"iVml00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionMiddleRightColor => {
                    friendly_name: "Middle Right Vision Input (Color)",
                    base_ascii: b"iVmr00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionBottomLeftColor => {
                    friendly_name: "Bottom Left Vision Input (Color)",
                    base_ascii: b"iVbl00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionBottomMiddleColor => {
                    friendly_name: "Bottom Middle Vision Input (Color)",
                    base_ascii: b"iVbm00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                VisionBottomRightColor => {
                    friendly_name: "Bottom Right Vision Input (Color)",
                    base_ascii: b"iVbr00",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                }
            }
        }
    };
}