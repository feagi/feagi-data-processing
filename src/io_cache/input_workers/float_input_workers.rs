use std::time::Instant;
use crate::data_types::neuron_data::{CorticalMappedXYZPNeuronData, NeuronTranslator, NeuronXYZPArrays};
use crate::data_types::RangedNormalizedF32;
use crate::error::DataProcessingError;
use crate::io_cache::{ChannelStatus, GroupingIndex, IOCacheWorker};
use crate::io_cache::ChannelIndex;
use crate::genome_definitions::identifiers::CorticalID;
use super::{InputCacheWorker};

pub trait InputFloatCacheWorker: InputCacheWorker<RangedNormalizedF32> {
    
}

//region Float Direct
pub struct FloatDirectWorker {
    last_data_update_time: Instant,
    channel_status: ChannelStatus,
    cortical_id_write_target: CorticalID, // yes, lets keep a copy here, this is too small to worry about borrowing shenanigans
    channel: ChannelIndex,
    last_float: RangedNormalizedF32,
}

impl IOCacheWorker<RangedNormalizedF32> for FloatDirectWorker {
    fn get_cached_data(&self) -> &RangedNormalizedF32 {
        &self.last_float
    }

    fn get_channel_status(&self) -> &ChannelStatus {
        &self.channel_status
    }

    fn get_grouping_index(&self) -> &GroupingIndex {
        &self.g
    }

    fn get_channel_index(&self) -> &ChannelIndex {
        todo!()
    }

    fn get_cortical_area_id(&self) -> &CorticalID {
        todo!()
    }
}

impl InputCacheWorker<RangedNormalizedF32> for FloatDirectWorker {
    fn write_to_cortical_mapped_xyzp_neuron_data(&self, translator: &dyn NeuronTranslator<RangedNormalizedF32>, write_target: &mut CorticalMappedXYZPNeuronData) -> Result<(), FeagiDataProcessingError> {
        if write_target.contains(&self.cortical_id_write_target) {
            let neurons_overwriting = write_target.borrow_mut(&self.cortical_id_write_target).unwrap(); // Due to earlier check, is safe
            translator.write_neuron_data_single_channel(self.last_float, neurons_overwriting, self.channel)?;
            return Ok(());
        }
        
        let mut neuron_data = NeuronXYZPArrays::new(2)?;
        translator.write_neuron_data_single_channel(self.last_float, &mut neuron_data, self.channel)?;
        write_target.insert(self.cortical_id_write_target, neuron_data);
        Ok(())
    }

    fn update_sensor_value(&mut self, sensor_value: RangedNormalizedF32) -> Result<(), FeagiDataProcessingError> {
        self.last_float = sensor_value;
        self.last_data_update_time = Instant::now();
        Ok(())
    }

    fn get_last_stored_sensor_value(&self) -> Result<&RangedNormalizedF32, FeagiDataProcessingError> {
        Ok(&self.last_float)
    }
}

impl InputFloatCacheWorker for FloatDirectWorker {
    
}

impl FloatDirectWorker {
    pub fn new(cortical_id_write_target: CorticalID, channel: ChannelIndex) -> Result<FloatDirectWorker, FeagiDataProcessingError> {
        Ok(FloatDirectWorker{
            last_data_update_time: Instant::now(),
            last_float: RangedNormalizedF32::new(0.0)?,
            cortical_id_write_target,
            channel
        })
    }
}
//endregion

