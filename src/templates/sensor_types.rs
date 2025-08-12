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
                #[doc = "Image camera input. Either alone or in the center of segmented/peripheral image camera setups"]
                ImageCameraCenter => {
                    friendly_name: "Center Image Camera Input",
                    base_ascii: b"iic400",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Top Left peripheral image camera input."]
                ImageCameraTopLeft => {
                    friendly_name: "Top Left Image Camera Input",
                    base_ascii: b"iic600",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Top Middle peripheral image camera input."]
                ImageCameraTopMiddle => {
                    friendly_name: "Top Middle Image Camera Input",
                    base_ascii: b"iic700",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Top Right peripheral image camera input."]
                ImageCameraTopRight => {
                    friendly_name: "Top Right Image Camera Input",
                    base_ascii: b"iic800",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Middle Left peripheral image camera input."]
                ImageCameraMiddleLeft => {
                    friendly_name: "Middle Left Image Camera Input",
                    base_ascii: b"iic300",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Middle Right peripheral image camera input."]
                ImageCameraMiddleRight => {
                    friendly_name: "Middle Right Image Camera Input",
                    base_ascii: b"iic400",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Bottom Left peripheral image camera input."]
                ImageCameraBottomLeft => {
                    friendly_name: "Bottom Left Image Camera Input",
                    base_ascii: b"iic000",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Bottom Middle peripheral image camera input."]
                ImageCameraBottomMiddle => {
                    friendly_name: "Bottom Middle Image Camera Input",
                    base_ascii: b"iic100",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                },
                #[doc = "Bottom Right peripheral image camera input."]
                ImageCameraBottomRight => {
                    friendly_name: "Bottom Right Image Camera Input",
                    base_ascii: b"iic200",
                    channel_dimension_range: SingleChannelDimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: NeuronCoderVariantType::ImageFrame,
                }
            }
        }
    };
}