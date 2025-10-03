use feagi_data_structures::FeagiDataError;
use feagi_data_structures::genomic::descriptors::{CorticalChannelIndex, CorticalGroupIndex};
use feagi_data_structures::genomic::{MotorCorticalType, SensorCorticalType};
use crate::caching::io_motor_cache::IOMotorCache;
use crate::caching::io_sensor_cache::IOSensorCache;
use crate::data_pipeline::{PipelineStageProperties, PipelineStagePropertyIndex};
use crate::data_pipeline::stage_properties::ImageSegmentorStageProperties;
use crate::data_types::descriptors::{GazeProperties, ImageFrameProperties, SegmentedImageFrameProperties};
use crate::data_types::Percentage4D;

pub fn loopback_absolute_gaze_to_segmentation<'a>(
    sensors: &'a mut IOSensorCache, 
    motors: &'a mut IOMotorCache<'a>,
    gaze_group: CorticalGroupIndex, 
    gaze_channel: CorticalChannelIndex,
    segmentation_group: CorticalGroupIndex,
    segmentation_channel: CorticalChannelIndex
) -> Result<feagi_data_structures::FeagiSignalIndex, FeagiDataError> {

    // Create a closure that captures references to sensors and motors
    // This now works because callbacks are lifetime-bounded, not 'static!
    let callback = move |_data: &()| {
        // Get current segmentator property
        const SENSOR_TYPE: SensorCorticalType = SensorCorticalType::ImageCameraCenterAbsolute;
        let temp: PipelineStagePropertyIndex = 0.into(); // TODO: FIX me!
        
        if let Ok(stage) = sensors.get_pipeline_stage_properties(SENSOR_TYPE, segmentation_group, segmentation_channel, temp) {
            if let Some(segmentator_property) = stage.as_any().downcast_ref::<ImageSegmentorStageProperties>() {
                let mut segmentator_property = segmentator_property.clone();
                
                // Get new gaze data
                const MOTOR_TYPE: MotorCorticalType = MotorCorticalType::GazeAbsoluteLinear;
                if let Ok(recent_output) = motors.try_read_postprocessed_cached_value(MOTOR_TYPE, gaze_group, gaze_channel) {
                    if let Ok(percentage_output) = recent_output.try_into() {
                        let percentage_output: Percentage4D = percentage_output;
                        let new_gaze = GazeProperties::new_4d(percentage_output);
                        
                        // Overwrite and apply
                        if let Ok(_) = segmentator_property.update_from_gaze(new_gaze) {
                            let segmentator_property: Box<dyn PipelineStageProperties + Send + Sync + 'static> = Box::new(segmentator_property);
                            let _ = sensors.try_updating_pipeline_stage(SENSOR_TYPE, segmentation_group, segmentation_channel, temp, segmentator_property);
                        }
                    }
                }
            }
        }
    };

    // Register the callback - it can now capture the cache references!
    motors.try_register_motor_callback(
        MotorCorticalType::GazeAbsoluteLinear, 
        gaze_group, 
        gaze_channel, 
        callback
    )
}