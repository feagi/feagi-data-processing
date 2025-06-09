use byteorder::{ByteOrder, LittleEndian};
use crate::error::DataProcessingError;
use super::{FeagiByteStructureCompatible, FeagiByteStructureType};

#[derive(Clone)]
pub struct FeagiByteStructure {
    bytes: Vec<u8>,
}

impl FeagiByteStructure {
    const GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES: usize = 2;
    const MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID: usize = 4;
    const MULTISTRUCT_STRUCT_COUNT_BYTE_SIZE: usize = 1;
    const MULTISTRUCT_PER_STRUCT_HEADER_SIZE_IN_BYTES: usize = 8;
    
    const SUPPORTED_VERSION_JSON: u8 = 1;
    const SUPPORTED_VERSION_MULTI_STRUCT: u8 = 1;
    const SUPPORTED_VERSION_NEURON_XYZP: u8 = 1;
    
    //region Constructors
    pub fn create_from_bytes(bytes: Vec<u8>) -> Result<FeagiByteStructure, DataProcessingError> {
        if bytes.len() < Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID {
            return Err(DataProcessingError::InvalidByteStructure(format!("Byte structure needs to be at least {} long to be considered valid. Given structure is only {} long!", Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID, bytes.len())));
        }
        _ = FeagiByteStructureType::try_from(bytes[0])?; // check if structure type is valid
        if bytes[1] == 0 {return Err(DataProcessingError::InvalidByteStructure("Byte structure cannot have version number of 0!".into()));}
        // NOTE: Other checks go here
        
        Ok(Self { bytes })
    }
    
    pub fn create_from_2_existing(a: &FeagiByteStructure, b: &FeagiByteStructure) -> Result<FeagiByteStructure, DataProcessingError> {
        // TODO Using vectors here is easier now, but we can squeeze a bit more performance by making a specific 2 slice system
        let structs = vec!(a, b);
        FeagiByteStructure::create_from_multiple_existing(structs)
    }
    
    pub fn create_from_multiple_existing(existing: Vec<&FeagiByteStructure>) -> Result<FeagiByteStructure, DataProcessingError> {
        
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
        Ok(FeagiByteStructure::build_multistruct_from_slices(slices))
    }
    
    pub fn create_from_compatible(object: Box<dyn FeagiByteStructureCompatible>) -> Result<FeagiByteStructure, DataProcessingError> {
        todo!()
    }
    
    pub fn create_from_multiple_compatible(objects: Vec<Box<dyn FeagiByteStructureCompatible>>) -> Result<FeagiByteStructure, DataProcessingError> {
        todo!()
    }
    //endregion
    
    //region Get Properties
    
    // NOTE: These functions have safety checks as they can be called externally
    
    pub fn try_get_structure_type(&self) -> Result<FeagiByteStructureType, DataProcessingError> {
        if self.bytes.len() == 0 {
            return Err(DataProcessingError::InvalidByteStructure(format!("Empty byte structure!")));
        }
        FeagiByteStructureType::try_from(self.bytes[0])
    }
    
    pub fn try_get_version(&self) -> Result<u8, DataProcessingError> {
        if self.bytes.len() < 2 {
            return Err(DataProcessingError::InternalError("Unable to get version information! Byte struct is too short!".into()))
        }
        Ok(self.bytes[1])
    }
    
    pub fn is_multistruct(&self) -> Result<bool, DataProcessingError> {
        Ok(FeagiByteStructureType::MultiStructHolder == self.try_get_structure_type()?)
    }
    
    pub fn contained_structure_count(&self) -> Result<usize, DataProcessingError> {
        if self.is_multistruct()? {
            self.verify_valid_multistruct_internal_count()?;
            return Ok(self.get_multistruct_contained_count());
        }
        Ok(1) // if not a multistruct, there's only one struct
    }
    
    pub fn get_ordered_object_types(&self) -> Result<Vec<FeagiByteStructureType>, DataProcessingError> {
        if self.is_multistruct()? {
            self.verify_valid_multistruct_internal_count();
            self.verify_valid_multistruct_internal_positionings_header();
            
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

    pub fn copy_out_as_byte_vector(&self) -> Vec<u8> {
        self.bytes.clone()
    }
    
    //endregion
    
    //region Verifications
    // NOTE: These functions are used to ensure internal data is reasonable. Not all have all
    // safety checks/
    
    fn verify_valid_multistruct_internal_count(&self) -> Result<(), DataProcessingError> {
        let len = self.bytes.len();
        if len < 3 {
            return Err(DataProcessingError::InternalError("Multistruct too short to hold contained struct count!!".into()))
        }
        if self.bytes[0] != FeagiByteStructureType::MultiStructHolder as u8 { // faster header check
            return Err(DataProcessingError::InternalError("Byte structure is not identified as a multistruct!".into()))
        }
        if self.bytes[2] != 0 {
            return Err(DataProcessingError::InternalError("Multistruct reports 0 contained structures!".into()))
        }
        Ok(())
    }
    
    fn verify_valid_multistruct_internal_positionings_header(&self) -> Result<(), DataProcessingError> {
        // We are assuming the internal structure count was already verified as existing and valid ( not 0)
        let len = self.bytes.len();
        let contained_struct_count = self.bytes[2] as usize;
        if contained_struct_count == 0 {
            return Err(DataProcessingError::InternalError("Multistruct reports 0 contained structures!".into())); // Explicitly check for this again because if we dont, we are going to underflow below
        }
        let header_size_bytes = contained_struct_count * Self::MULTISTRUCT_PER_STRUCT_HEADER_SIZE_IN_BYTES;
        if len <  Self::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + Self::MULTISTRUCT_STRUCT_COUNT_BYTE_SIZE + header_size_bytes {
            return Err(DataProcessingError::InvalidByteStructure("Multi Struct too short to hold contained positionings header!".into()))
        } 
        
        let len = len as u32;
        let mut struct_header_start_index = Self::GLOBAL_BYTE_HEADER_BYTE_SIZE_IN_BYTES + Self::MULTISTRUCT_STRUCT_COUNT_BYTE_SIZE;
        for c in 0..contained_struct_count {
            let struct_start_index = LittleEndian::read_u32(&self.bytes[struct_header_start_index..struct_header_start_index + 4]);
            let struct_length = LittleEndian::read_u32(&self.bytes[struct_header_start_index + 4..struct_header_start_index + 8]);
            if struct_start_index + struct_length > len {
                return Err(DataProcessingError::InvalidByteStructure("Multi Struct too short to hold all reported contained structures!".into()))
            }
            struct_header_start_index += Self::MULTISTRUCT_PER_STRUCT_HEADER_SIZE_IN_BYTES;
        }
        Ok(())
    }
    
    fn verify_valid_multistruct_internal_slices_header_and_size(&self, internal_slices: &Vec<&[u8]>) -> Result<(), DataProcessingError> {
        for slice in internal_slices {
            if slice.len() < Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID {
                return Err(DataProcessingError::InvalidByteStructure("Multi Struct contains a internal structure too small to be valid!".into()))
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

    pub fn ensure_capacity_of_at_least(&mut self, size: usize) -> Result<(), DataProcessingError> {
        if size < Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID {
            return Err(DataProcessingError::InvalidInputBounds(format!("Cannot set capacity to less than minimum required capacity of {}!", Self::MINIMUM_LENGTH_TO_BE_CONSIDERED_VALID)));
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
        output_bytes[3] = slice_count as u8;
        
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

pub fn try_get_version_from_bytes(bytes: &[u8]) -> Result<u8, DataProcessingError> {
    if bytes.len() < 2 {
        return Err(DataProcessingError::InvalidByteStructure("Cannot ascertain type of 0/1 long byte array!".into()))
    }
    Ok(bytes[1])
}

pub fn verify_matching_structure_type_and_version(feagi_byte_structure: &FeagiByteStructure, expected_type: FeagiByteStructureType, expected_version: u8) -> Result<(), DataProcessingError> {
    if feagi_byte_structure.try_get_structure_type()? != expected_type {
        return Err(DataProcessingError::InvalidByteStructure(format!(
            "Given structure of type {} cannot be instantiated for entity corresponding to type {}!", feagi_byte_structure.try_get_structure_type().unwrap() as u8, expected_type as u8)));
    }
    if feagi_byte_structure.try_get_version()? != expected_version {
        return Err(DataProcessingError::InvalidByteStructure(format!(
            "Given structure of version {} cannot be instantiated for entity corresponding to version {}!", feagi_byte_structure.try_get_version()?, expected_version)));
    }
    Ok(())
}