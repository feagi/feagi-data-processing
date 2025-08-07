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
                #[doc = "Vision input. Either alone or in the center of segmented/peripheral vision setups"]
                VisionCenter => {
                    friendly_name: "Center Vision Input",
                    base_ascii: b"iic400",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Top Left peripheral vision input."]
                VisionTopLeft => {
                    friendly_name: "Top Left Vision Input",
                    base_ascii: b"iic600",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Top Middle peripheral vision input."]
                VisionTopMiddle => {
                    friendly_name: "Top Middle Vision Input",
                    base_ascii: b"iic700",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Top Right peripheral vision input."]
                VisionTopRight => {
                    friendly_name: "Top Right Vision Input",
                    base_ascii: b"iic800",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Middle Left peripheral vision input."]
                VisionMiddleLeft => {
                    friendly_name: "Middle Left Vision Input",
                    base_ascii: b"iic300",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Middle Right peripheral vision input."]
                VisionMiddleRight => {
                    friendly_name: "Middle Right Vision Input",
                    base_ascii: b"iic400",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Bottom Left peripheral vision input."]
                VisionBottomLeft => {
                    friendly_name: "Bottom Left Vision Input",
                    base_ascii: b"iic000",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Bottom Middle peripheral vision input."]
                VisionBottomMiddle => {
                    friendly_name: "Bottom Middle Vision Input",
                    base_ascii: b"iic100",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Bottom Right peripheral vision input."]
                VisionBottomRight => {
                    friendly_name: "Bottom Right Vision Input",
                    base_ascii: b"iic200",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                }
            }
        }
    };
}