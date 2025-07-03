use byteorder::{ByteOrder, LittleEndian};
use crate::error::{FeagiBytesError, FeagiDataProcessingError, IODataError};
use crate::io_data::JsonStructure;
use crate::neuron_data::CorticalMappedXYZPNeuronData; 
use super::FeagiByteStructureType;
use super::FeagiByteStructureCompatible;

#[derive(Clone)]
pub struct FeagiByteStructure {
    bytes: Vec<u8>,
}

impl FeagiByteStructure {
    pub const GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES: usize = 2;
    pub const MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID: usize = 4;
    pub const MULTISTRUCT_STRUCT_COUNT_BYTE_SIZE: usize = 1;
    pub const MULTISTRUCT_PER_STRUCT_HEADER_SIZE_IN_BYTES: usize = 8;

    pub const SUPPORTED_VERSION_JSON: u8 = 1;
    pub const SUPPORTED_VERSION_MULTI_STRUCT: u8 = 1;
    pub const SUPPORTED_VERSION_NEURON_XYZP: u8 = 1;
    
    //region Constructors
    pub fn create_from_bytes(bytes: Vec<u8>) -> Result<FeagiByteStructure, FeagiDataProcessingError> {
        if bytes.len() < Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID {
            return Err(FeagiBytesError::UnableToValidateBytes(format!("Byte structure needs to be at least {} long to be considered valid. Given structure is only {} long!", Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID, bytes.len())).into());
        }
        _ = FeagiByteStructureType::try_from(bytes[0])?; // check if structure type is valid
        if bytes[1] == 0 {return Err(FeagiBytesError::UnableToValidateBytes("Byte structure cannot have version number of 0!".into()).into());}
        // NOTE: Other checks go here
        
        Ok(Self { bytes })
    }
    
    pub fn create_from_2_existing(a: &FeagiByteStructure, b: &FeagiByteStructure) -> Result<FeagiByteStructure, FeagiDataProcessingError> {
        // TODO Using vectors here is easier now, but we can squeeze a bit more performance by making a specific 2 slice system
        let structs = vec!(a, b);
        FeagiByteStructure::create_from_multiple_existing(structs)
    }
    
    pub fn create_from_multiple_existing(existing: Vec<&FeagiByteStructure>) -> Result<FeagiByteStructure, FeagiDataProcessingError> {
        
        if existing.is_empty() {
            return Err(IODataError::InvalidParameters("You must specify at least one byte structure to put into a multistruct!".into()).into());
        }
        
        if existing.len() == 1 {
            // No need to make a whole new structure, just copy this
            return Ok(existing[0].clone());
        }
        
        // Break apart any input multistructs, we don't want nesting. Assuming FeagiByteStructures are Valid
        let mut slices: Vec<&[u8]> = Vec::new();
        for input in existing {
            if input.is_multistruct()? {
                slices.extend(input.get_all_multistruct_internal_slices())
            }
            else {
                slices.push(input.borrow_data_as_slice())
            }
        }

        if slices.len() > 255 {
            // wtf are you doing
            return Err(IODataError::InvalidParameters("The maximum number of structures that can exist in a multistruct is 255!".into()).into());
        }
        
        Ok(FeagiByteStructure::build_multistruct_from_slices(slices))
    }
    
    pub fn create_from_compatible(object: Box<dyn FeagiByteStructureCompatible>) -> Result<FeagiByteStructure, FeagiDataProcessingError> {
        // Essentially just an alias
        object.as_new_feagi_byte_structure()
    }
    
    /*
    pub fn create_from_multiple_compatible(objects: Vec<Box<dyn FeagiByteStructureCompatible>>) -> Result<FeagiByteStructure, FeagiDataProcessingError> {
        todo!()
    }
     */
    //endregion
    
    //region static safety checks
    pub(crate) fn verify_matching_structure_type_and_version(feagi_byte_structure: &FeagiByteStructure, expected_type: FeagiByteStructureType, expected_version: u8) -> Result<(), FeagiDataProcessingError> {
        if feagi_byte_structure.try_get_structure_type()? != expected_type {
            return Err(FeagiBytesError::UnableToValidateBytes(format!(
                "Given structure of type {} cannot be instantiated for entity corresponding to type {}!", feagi_byte_structure.try_get_structure_type().unwrap() as u8, expected_type as u8)).into());
        }
        if feagi_byte_structure.try_get_version()? != expected_version {
            return Err(FeagiBytesError::UnableToValidateBytes(format!(
                "Given structure of version {} cannot be instantiated for entity corresponding to version {}!", feagi_byte_structure.try_get_version()?, expected_version)).into());
        }
        Ok(())
    }
    //endregion
    
    //region Get Properties
    
    // NOTE: These functions have safety checks as they can be called externally
    
    pub fn try_get_structure_type(&self) -> Result<FeagiByteStructureType, FeagiDataProcessingError> {
        if self.bytes.len() == 0 {
            return Err(FeagiDataProcessingError::InternalError("Empty byte structure!".to_string())); // This shouldn't be possible as this struct should be checked before being created
        }
        FeagiByteStructureType::try_from(self.bytes[0])
    }
    
    pub fn try_get_version(&self) -> Result<u8, FeagiDataProcessingError> {
        if self.bytes.len() < 2 {
            return Err(FeagiDataProcessingError::InternalError("Unable to get version information! Byte struct is too short!".into()))
        }
        Ok(self.bytes[1])
    }
    
    pub fn is_multistruct(&self) -> Result<bool, FeagiDataProcessingError> {
        Ok(FeagiByteStructureType::MultiStructHolder == self.try_get_structure_type()?)
    }
    
    pub fn contained_structure_count(&self) -> Result<usize, FeagiDataProcessingError> {
        if self.is_multistruct()? {
            self.verify_valid_multistruct_internal_count()?;
            return Ok(self.get_multistruct_contained_count());
        }
        Ok(1) // if not a multistruct, there's only one struct
    }
    
    pub fn get_ordered_object_types(&self) -> Result<Vec<FeagiByteStructureType>, FeagiDataProcessingError> {
        if self.is_multistruct()? {
            self.verify_valid_multistruct_internal_count()?;
            self.verify_valid_multistruct_internal_positionings_header()?;
            
            let struct_slices = self.get_all_multistruct_internal_slices();
            self.verify_valid_multistruct_internal_slices_header_and_size(&struct_slices)?;
            
            let mut output: Vec<FeagiByteStructureType> = Vec::with_capacity(struct_slices.len());
            for slice in struct_slices {
                output.push(FeagiByteStructureType::try_from(slice[0])?);
            };
            return Ok(output);
        }
        Ok(vec![self.try_get_structure_type()?])
    }

    pub fn copy_out_single_byte_structure_from_multistruct(&self, index: usize) -> Result<FeagiByteStructure, FeagiDataProcessingError> {
        if !self.is_multistruct()? {
            return Ok(self.clone());
        }
        if index > self.contained_structure_count()? {
            return Err(IODataError::InvalidParameters(format!("Given struct index {} is out of bounds given this multistruct only contains {} elements!", index, self.contained_structure_count()?)).into());
        }
        Ok(FeagiByteStructure::create_from_bytes(
            self.get_multistruct_specific_slice(index).to_vec()
        )?)
    }
    
    pub fn copy_out_single_object_from_single_struct(&self) -> Result<Box<dyn FeagiByteStructureCompatible>, FeagiDataProcessingError> {
        let this_struct_type = self.try_get_structure_type()?;
        if this_struct_type == FeagiByteStructureType::MultiStructHolder {
            return Err(FeagiBytesError::IncompatibleByteUse("Cannot return a multistruct holding multiple structs as a single object!".into()).into())
        }
        
        // Factory pattern to create the appropriate concrete type based on structure type
        match this_struct_type {
            FeagiByteStructureType::JSON => {
                Ok(Box::new(JsonStructure::new_from_feagi_byte_structure(self)?))
            },
            FeagiByteStructureType::NeuronCategoricalXYZP => {
                Ok(Box::new(CorticalMappedXYZPNeuronData::new_from_feagi_byte_structure(self)?))
            },
            FeagiByteStructureType::MultiStructHolder => {
                // This case is already handled above, but included for completeness
                Err(FeagiBytesError::IncompatibleByteUse("Cannot return a multistruct holding multiple structs as a single object!".into()).into())
            }
            _ => {
                Err(FeagiDataProcessingError::InternalError(format!("Missing export definition for FBS object type {}!", this_struct_type)))
            }
        }
    }
    
    pub fn copy_out_single_object_from_multistruct(&self, index: usize) -> Result<Box<dyn FeagiByteStructureCompatible>, FeagiDataProcessingError> {
        // TODO this method is slow, we should have a dedicated create from byte slice for FeagiByteStructureCompatible
        if !self.is_multistruct()? {
            return Err(FeagiBytesError::UnableToDeserializeBytes("Cannot deserialize this object as a multistruct when it is not!".into()).into())
        }
        let internal = self.copy_out_single_byte_structure_from_multistruct(index)?;
        internal.copy_out_single_object_from_single_struct()
    }

    pub fn copy_out_as_byte_vector(&self) -> Vec<u8> {
        self.bytes.clone()
    }
    
    //endregion
    
    //region Verifications
    // NOTE: These functions are used to ensure internal data is reasonable. Not all have all
    // safety checks/
    
    fn verify_valid_multistruct_internal_count(&self) -> Result<(), FeagiDataProcessingError> {
        let len = self.bytes.len();
        if len < Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID {
            return Err(FeagiDataProcessingError::InternalError("byte structure too short!".into()))
        }
        if self.bytes[0] != FeagiByteStructureType::MultiStructHolder as u8 { // faster header check
            return Err(FeagiBytesError::UnableToValidateBytes("Byte structure is not identified as a multistruct!".into()).into())
        }
        if self.bytes[2] == 0 {
            return Err(FeagiDataProcessingError::InternalError("Multistruct reports 0 contained structures!".into()))
        }
        Ok(())
    }
    
    fn verify_valid_multistruct_internal_positionings_header(&self) -> Result<(), FeagiDataProcessingError> {
        // We are assuming the internal structure count was already verified as existing and valid ( not 0)
        let len = self.bytes.len();
        let contained_struct_count = self.bytes[2] as usize;
        if contained_struct_count == 0 {
            return Err(FeagiDataProcessingError::InternalError("Multistruct reports 0 contained structures!".into())); // Explicitly check for this again because if we dont, we are going to underflow below
        }
        let header_size_bytes = contained_struct_count * Self::MULTISTRUCT_PER_STRUCT_HEADER_SIZE_IN_BYTES;
        if len <  Self::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + Self::MULTISTRUCT_STRUCT_COUNT_BYTE_SIZE + header_size_bytes {
            return Err(FeagiDataProcessingError::InternalError("Multi Struct too short to hold contained positionings header!".into()))
        } 
        
        let len = len as u32;
        let mut struct_header_start_index = Self::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + Self::MULTISTRUCT_STRUCT_COUNT_BYTE_SIZE;
        for _ in 0..contained_struct_count {
            let struct_start_index = LittleEndian::read_u32(&self.bytes[struct_header_start_index..struct_header_start_index + 4]);
            let struct_length = LittleEndian::read_u32(&self.bytes[struct_header_start_index + 4..struct_header_start_index + 8]);
            if struct_start_index + struct_length > len {
                return Err(FeagiDataProcessingError::InternalError("Multi Struct too short to hold all reported contained structures!".into()))
            }
            struct_header_start_index += Self::MULTISTRUCT_PER_STRUCT_HEADER_SIZE_IN_BYTES;
        }
        Ok(())
    }
    
    fn verify_valid_multistruct_internal_slices_header_and_size(&self, internal_slices: &Vec<&[u8]>) -> Result<(), FeagiDataProcessingError> {
        for slice in internal_slices {
            if slice.len() < Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID {
                return Err(FeagiDataProcessingError::InternalError("Multi Struct contains a internal structure too small to be valid!".into()))
            }
            FeagiByteStructureType::try_from(slice[0])?;
        }
        Ok(())
    }
    
    //endregion
    
    //region Borrow Data

    pub fn borrow_data_as_slice(&self) -> &[u8] {
        &self.bytes
    }

    pub fn borrow_data_as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    pub fn borrow_data_as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }
    
    //endregion
    
    //region Interactions with Internal Vector

    pub fn get_wasted_capacity_count(&self) -> usize {
        self.bytes.capacity() - self.bytes.len()
    }

    pub fn get_utilized_capacity_percentage(&self) -> f32 {
        (self.bytes.len() as f32 / self.bytes.capacity() as f32) * 100.0
    }

    pub fn ensure_capacity_of_at_least(&mut self, size: usize) -> Result<(), FeagiDataProcessingError> {
        if size < Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID {
            return Err(IODataError::InvalidParameters(format!("Cannot set capacity to less than minimum required capacity of {}!", Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID)).into());
        }

        if self.bytes.capacity() < size {
            //self.bytes.reserve(size - self.bytes.capacity());
        }
        Ok(())
    }

    pub fn shed_wasted_capacity(&mut self) {
        self.bytes.shrink_to_fit();
    }

    pub fn reset_write_index(&mut self) {
        self.bytes.truncate(0);
    }
    
    //endregion
    
    //region Internals
    // WARNING: Most of these functions do not check for byte structure validity, be cautious

    fn build_multistruct_from_slices(all_slices: Vec<&[u8]>) -> FeagiByteStructure {
        // NOTE: does not check if internal slices are sensible
        let slice_count = all_slices.len();
        let mut total_slices_byte_count: usize = 0;
        for slice in &all_slices {
            total_slices_byte_count += slice.len();
        };
        let header_output_length = Self::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + Self::MULTISTRUCT_STRUCT_COUNT_BYTE_SIZE +
            (Self::MULTISTRUCT_PER_STRUCT_HEADER_SIZE_IN_BYTES * slice_count);
        
        let total_output_length = header_output_length + total_slices_byte_count;
        
        // Write output data
        let mut output_bytes: Vec<u8> = Vec::with_capacity(total_output_length);
        output_bytes.resize(total_output_length, 0);
        
        // global header
        output_bytes[0] = FeagiByteStructureType::MultiStructHolder as u8;
        output_bytes[1] = Self::SUPPORTED_VERSION_MULTI_STRUCT;
        
        // struct count subheader
        output_bytes[2] = slice_count as u8;
        
        // subheader + data
        let mut subheader_write_index: usize =  Self::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + Self::MULTISTRUCT_STRUCT_COUNT_BYTE_SIZE;
        let mut data_write_index: usize = header_output_length; // start right after header
        
        for slice in &all_slices {
            let slice_length = slice.len();
            
            // sub header
            output_bytes[subheader_write_index.. subheader_write_index + 4].copy_from_slice(
                &(data_write_index as u32).to_le_bytes() // location
            );
            output_bytes[subheader_write_index + 4.. subheader_write_index + 8].copy_from_slice(
                &(slice_length as u32).to_le_bytes() // length
            );
            
            // data
            output_bytes[data_write_index..data_write_index + slice_length].copy_from_slice(
                slice
            );

            subheader_write_index += Self::MULTISTRUCT_PER_STRUCT_HEADER_SIZE_IN_BYTES;
            data_write_index += slice_length;
        };
        
        // Skip any checks and instantiate directly
        FeagiByteStructure {bytes: output_bytes}
    }
    
    fn get_multistruct_contained_count(&self) -> usize {
        // NOTE no safety checks, make sure your vector is a valid multistruct
        self.bytes[2] as usize
    }

    fn get_multistruct_specific_slice(&self, index: usize) -> &[u8] {
        // WARNING: No boundary checks, be careful!
        let reading_offset = Self::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + Self::MULTISTRUCT_STRUCT_COUNT_BYTE_SIZE +
            (index * Self::MULTISTRUCT_PER_STRUCT_HEADER_SIZE_IN_BYTES);
        let struct_start = LittleEndian::read_u32(&self.bytes[reading_offset..reading_offset + 4]) as usize;
        let struct_length = LittleEndian::read_u32(&self.bytes[reading_offset + 4..reading_offset + 8]) as usize;
        &self.bytes[struct_start..struct_start + struct_length]
    }
    
    fn get_all_multistruct_internal_slices(&self) -> Vec<&[u8]> {
        let mut output = Vec::with_capacity(self.get_multistruct_contained_count());
        for i in 0..self.get_multistruct_contained_count() {
            output.push(self.get_multistruct_specific_slice(i));
        }
        output
    }
    
    
    //endregion
    
}

pub fn try_get_version_from_bytes(bytes: &[u8]) -> Result<u8, FeagiDataProcessingError> {
    if bytes.len() <  FeagiByteStructure::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID {
        return Err(FeagiBytesError::UnableToDeserializeBytes("Structure too short to be a Feagi Byte Structure".into()).into());
    }
    Ok(bytes[1])
}