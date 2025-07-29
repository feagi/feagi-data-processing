/// Definition File for Sensors (Input Processing Units)
#[macro_export]
macro_rules! sensor_definition {
    (iopu_definition!()) =>
    {
    SensorCorticalType {
        Infrared => {
            friendly_name: "Infrared Sensor",
            base_ascii: b"iinf00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(Some(1), Some(1), Some(1)),
            io_variants: [IOTypeVariant::F32],
            encoder_type: NeuronCoderVariantType::Normalized0To1F32,
        },
        ReverseInfrared => {
            friendly_name: "Reverse Infrared Sensor",
            base_ascii: b"iiif00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(Some(1), Some(1), Some(1)),
            io_variants: [IOTypeVariant::F32],
            encoder_type: NeuronCoderVariantType::Normalized0To1F32,
        },
        DigitalGPIOInput => {
            friendly_name: "GPIO Digital Input",
            base_ascii: b"idgp00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(Some(1), Some(1), Some(1)),
            io_variants: [IOTypeVariant::F32],
            encoder_type: NeuronCoderVariantType::Normalized0To1F32,
        },
        Proximity => {
            friendly_name: "Proximity",
            base_ascii: b"ipro00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(Some(1), Some(1), None),
            io_variants: [IOTypeVariant::F32],
            encoder_type: NeuronCoderVariantType::Normalized0To1F32,
        },
        Shock => {
            friendly_name: "Shock",
            base_ascii: b"ishk00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(Some(1), Some(1), Some(1)),
            io_variants: [IOTypeVariant::F32],
            encoder_type: NeuronCoderVariantType::Normalized0To1F32,
        },
        
        
        VisionCenterGray => {
            friendly_name: "Center Vision Input (Grayscale)",
            base_ascii: b"ivcc00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionTopLeftGray => {
            friendly_name: "Top Left Vision Input (Grayscale)",
            base_ascii: b"ivtl00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionTopMiddleGray => {
            friendly_name: "Top Middle Vision Input (Grayscale)",
            base_ascii: b"ivtm00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionTopRightGray => {
            friendly_name: "Top Right Vision Input (Grayscale)",
            base_ascii: b"ivtr00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionMiddleLeftGray => {
            friendly_name: "Middle Left Vision Input (Grayscale)",
            base_ascii: b"ivml00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionMiddleRightGray => {
            friendly_name: "Middle Right Vision Input (Grayscale)",
            base_ascii: b"ivmr00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionBottomLeftGray => {
            friendly_name: "Bottom Left Vision Input (Grayscale)",
            base_ascii: b"ivbl00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionBottomMiddleGray => {
            friendly_name: "Bottom Middle Vision Input (Grayscale)",
            base_ascii: b"ivbm00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionBottomRightGray => {
            friendly_name: "Bottom Right Vision Input (Grayscale)",
            base_ascii: b"ivbr00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionCenterColor => {
            friendly_name: "Center Vision Input (Color)",
            base_ascii: b"iVcc00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionTopLeftColor => {
            friendly_name: "Top Left Vision Input (Color)",
            base_ascii: b"iVtl00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionTopMiddleColor => {
            friendly_name: "Top Middle Vision Input (Color)",
            base_ascii: b"iVtm00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionTopRightColor => {
            friendly_name: "Top Right Vision Input (Color)",
            base_ascii: b"iVtr00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionMiddleLeftColor => {
            friendly_name: "Middle Left Vision Input (Color)",
            base_ascii: b"iVml00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionMiddleRightColor => {
            friendly_name: "Middle Right Vision Input (Color)",
            base_ascii: b"iVmr00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionBottomLeftColor => {
            friendly_name: "Bottom Left Vision Input (Color)",
            base_ascii: b"iVbl00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionBottomMiddleColor => {
            friendly_name: "Bottom Middle Vision Input (Color)",
            base_ascii: b"iVbm00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        VisionBottomRightColor => {
            friendly_name: "Bottom Right Vision Input (Color)",
            base_ascii: b"iVbr00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::ImageFrame, IOTypeVariant::SegmentedImageFrame],
            encoder_type: NeuronCoderVariantType::ImageFrame,
        },
        Miscellaneous => {
            friendly_name: "Miscellaneous",
            base_ascii: b"imis00",
            channel_dimensions: SingleChannelDimensionsRequirements::new(None, None, None),
            io_variants: [IOTypeVariant::F32],
            encoder_type: NeuronCoderVariantType::Normalized0To1F32,
        }
    }    
    }
}