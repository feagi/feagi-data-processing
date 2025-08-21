/// Definition File for Sensors (Input Processing Units)
#[macro_export]
macro_rules! sensor_definition {
    ($callback:ident) => {
        $callback! {
            SensorCorticalType {
                
                //region 0 - 1 Linear Float
                
                #[doc = "Infrared distance sensor for object detection"]
                Infrared => {
                    friendly_name: "Infrared Sensor",
                    snake_case_identifier: "infrared",
                    base_ascii: b"iinf00",
                    channel_dimension_range: DimensionRange::new(1..2, 1..2, 1..2),
                    default_coder_type: Some(NeuronCoderType::F32Normalized0To1_Linear),
                },
                #[doc = "Inverted infrared sensor that provides reverse object detection readings."]
                ReverseInfrared => {
                    friendly_name: "Reverse Infrared Sensor",
                    snake_case_identifier: "reverse_infrared",
                    base_ascii: b"iiif00",
                    channel_dimension_range: DimensionRange::new(1..2, 1..2, 1..2),
                    default_coder_type: Some(NeuronCoderType::F32Normalized0To1_Linear),
                },
                #[doc = "Digital GPIO input pin for reading binary signals (high/low states)."]
                DigitalGPIOInput => {
                    friendly_name: "GPIO Digital Input",
                    snake_case_identifier: "gpio_digital_input",
                    base_ascii: b"idgp00",
                    channel_dimension_range: DimensionRange::new(1..2, 1..2, 1..2),
                    default_coder_type: Some(NeuronCoderType::F32Normalized0To1_Linear),
                },
                #[doc = "Proximity sensor for detecting nearby objects and measuring distances."]
                Proximity => {
                    friendly_name: "Proximity",
                    snake_case_identifier: "proximity",
                    base_ascii: b"ipro00",
                    channel_dimension_range: DimensionRange::new(1..2, 1..2, 1..u32::MAX),
                    default_coder_type: Some(NeuronCoderType::F32Normalized0To1_Linear),
                },
                #[doc = "Shock sensor for sensing 'pain'"]
                Shock => {
                    friendly_name: "Shock",
                    snake_case_identifier: "shock",
                    base_ascii: b"ishk00",
                    channel_dimension_range: DimensionRange::new(1..2, 1..2, 1..2),
                    default_coder_type: Some(NeuronCoderType::F32Normalized0To1_Linear),
                },
                #[doc = "Battery level sensor for monitoring power remaining."]
                Battery => {
                    friendly_name: "Battery Gauge",
                    snake_case_identifier: "battery_gauge",
                    base_ascii: b"ibat00",
                    channel_dimension_range: DimensionRange::new(1..2, 1..2, 1..u32::MAX),
                    default_coder_type: Some(NeuronCoderType::F32Normalized0To1_Linear),
                },
                
                //endregion
                
                //region -1 -1 Split Sign Float
                
                #[doc = "Servo position feedback sensor for monitoring actuator positions."]
                ServoPosition => {
                    friendly_name: "Servo Position",
                    snake_case_identifier: "servo_position",
                    base_ascii: b"isvp00",
                    channel_dimension_range: DimensionRange::new(1..2, 1..2, 1..u32::MAX),
                    default_coder_type: Some(NeuronCoderType::F32NormalizedM1To1_SplitSignDivided),
                },
                
                //endregion
                
                //region ImageFrame
                
                #[doc = "Image camera input. Either alone or in the center of segmented/peripheral image camera setups"]
                ImageCameraCenter => {
                    friendly_name: "Center Image Camera Input",
                    snake_case_identifier: "center_image_camera_input",
                    base_ascii: b"iic400",
                    channel_dimension_range: DimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: Some(NeuronCoderType::ImageFrame),
                },
                #[doc = "Top Left peripheral image camera input."]
                ImageCameraTopLeft => {
                    friendly_name: "Top Left Image Camera Input",
                    snake_case_identifier: "top_left_image_camera_input",
                    base_ascii: b"iic600",
                    channel_dimension_range: DimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: None,
                },
                #[doc = "Top Middle peripheral image camera input."]
                ImageCameraTopMiddle => {
                    friendly_name: "Top Middle Image Camera Input",
                    snake_case_identifier: "top_middle_image_camera_input",
                    base_ascii: b"iic700",
                    channel_dimension_range: DimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: None,
                },
                #[doc = "Top Right peripheral image camera input."]
                ImageCameraTopRight => {
                    friendly_name: "Top Right Image Camera Input",
                    snake_case_identifier: "top_right_image_camera_input",
                    base_ascii: b"iic800",
                    channel_dimension_range: DimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: None,
                },
                #[doc = "Middle Left peripheral image camera input."]
                ImageCameraMiddleLeft => {
                    friendly_name: "Middle Left Image Camera Input",
                    snake_case_identifier: "middle_left_image_camera_input",
                    base_ascii: b"iic300",
                    channel_dimension_range: DimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: None,
                },
                #[doc = "Middle Right peripheral image camera input."]
                ImageCameraMiddleRight => {
                    friendly_name: "Middle Right Image Camera Input",
                    snake_case_identifier: "middle_right_image_camera_input",
                    base_ascii: b"iic400",
                    channel_dimension_range: DimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: None,
                },
                #[doc = "Bottom Left peripheral image camera input."]
                ImageCameraBottomLeft => {
                    friendly_name: "Bottom Left Image Camera Input",
                    snake_case_identifier: "bottom_left_image_camera_input",
                    base_ascii: b"iic000",
                    channel_dimension_range: DimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: None,
                },
                #[doc = "Bottom Middle peripheral image camera input."]
                ImageCameraBottomMiddle => {
                    friendly_name: "Bottom Middle Image Camera Input",
                    snake_case_identifier: "bottom_middle_image_camera_input",
                    base_ascii: b"iic100",
                    channel_dimension_range: DimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: None,
                },
                #[doc = "Bottom Right peripheral image camera input."]
                ImageCameraBottomRight => {
                    friendly_name: "Bottom Right Image Camera Input",
                    snake_case_identifier: "bottom_right_image_camera_input",
                    base_ascii: b"iic200",
                    channel_dimension_range: DimensionRange::new(1..u32::MAX, 1..u32::MAX, 1..5),
                    default_coder_type: None,
                }
                //endregion
            }
        }
    };
}