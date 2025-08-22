use crate::error::{FeagiDataProcessingError, IODataError};
use crate::io_data::IOTypeVariant;
use crate::io_processing::StreamCacheProcessor;

pub fn verify_sensor_chain(cache_processors: &Vec<Box<dyn StreamCacheProcessor + Sync + Send>>) -> Result<(), FeagiDataProcessingError> {
    let number_of_processors = cache_processors.len();

    if number_of_processors == 0 {
        return Err(IODataError::InvalidParameters("Processor Runner cannot have 0 Cache Processors!".into()).into())
    }

    // Ensure data can pass between processing
    for processer_index in 0..number_of_processors - 1  {
        let first = &cache_processors[processer_index];
        let second = &cache_processors[processer_index + 1];
        if first.get_output_data_type() != second.get_input_data_type() {
            return Err(IODataError::InvalidParameters(format!("Given cache processor at index {} has output type {}, which does not match the input type of cache processor at index {} or type {}!",
                                                              processer_index, first.get_output_data_type(), processer_index + 1,  second.get_input_data_type()).into()).into());
        }
    };

    Ok(())
}

pub fn verify_sensor_chain_and_encoder(cache_processors: &Vec<Box<dyn StreamCacheProcessor + Sync + Send>>, type_accepted_by_encoder: &IOTypeVariant)
    -> Result<(), FeagiDataProcessingError> {
    
    verify_sensor_chain(cache_processors)?;

    if &cache_processors.last().unwrap().get_output_data_type() != type_accepted_by_encoder {
        return Err(IODataError::InvalidParameters(format!("Given cache processor at index {} has output type {}, which does not match the input type of the encoder taking type {}!",
                                                          cache_processors.len() - 1, cache_processors.last().unwrap().get_output_data_type(), type_accepted_by_encoder).into()).into());
    }

    Ok(())
}
