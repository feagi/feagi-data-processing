use feagi_core_data_structures_and_processing::miscellaneous_types::json_structure::JsonStructure;
use feagi_core_data_structures_and_processing::neuron_data::neuron_mappings::CorticalMappedXYZPNeuronData;
use feagi_core_data_structures_and_processing::neuron_data::neuron_arrays::NeuronXYZPArrays;
use feagi_core_data_structures_and_processing::neuron_data::neurons::NeuronXYZP;
use feagi_core_data_structures_and_processing::cortical_data::CorticalID;
use feagi_core_data_structures_and_processing::byte_structures::feagi_byte_structure::FeagiByteStructure;
use feagi_core_data_structures_and_processing::byte_structures::{FeagiByteStructureCompatible, FeagiByteStructureType};
use serde_json::json;

#[test]
fn test_combined_neuron_json_multistruct_serialize_deserialize() {
    // Create JSON structure
    let json_data = json!({
        "experiment_name": "Neural Network Test",
        "parameters": {
            "learning_rate": 0.001,
            "batch_size": 32,
            "epochs": 100
        },
        "metadata": {
            "created_at": "2024-01-01T00:00:00Z",
            "version": "1.0.0"
        }
    });
    let json_structure = JsonStructure::from_json_value(json_data.clone());

    // Create neuron structure (similar to the neuron tests)
    let cortical_id_a = CorticalID::from_str("AAAAAA").unwrap();
    let neuron_a_1 = NeuronXYZP::new(10, 20, 30, 0.75);
    let neuron_a_2 = NeuronXYZP::new(40, 50, 60, 0.25);
    let mut neurons_a = NeuronXYZPArrays::new(2).unwrap();
    neurons_a.add_neuron(&neuron_a_1);
    neurons_a.add_neuron(&neuron_a_2);

    let cortical_id_b = CorticalID::from_str("BBBBBB").unwrap();
    let neuron_b_1 = NeuronXYZP::new(100, 200, 300, 0.8);
    let mut neurons_b = NeuronXYZPArrays::new(1).unwrap();
    neurons_b.add_neuron(&neuron_b_1);

    let mut neuron_mappings = CorticalMappedXYZPNeuronData::new();
    neuron_mappings.insert(cortical_id_a, neurons_a);
    neuron_mappings.insert(cortical_id_b, neurons_b);

    // Convert both to individual FeagiByteStructures
    let json_byte_structure = json_structure.as_new_feagi_byte_structure().unwrap();
    let neuron_byte_structure = neuron_mappings.as_new_feagi_byte_structure().unwrap();

    // Verify individual structures have correct types
    assert_eq!(json_byte_structure.try_get_structure_type().unwrap(), FeagiByteStructureType::JSON);
    assert_eq!(neuron_byte_structure.try_get_structure_type().unwrap(), FeagiByteStructureType::NeuronCategoricalXYZP);

    // Create combined multi-struct
    let combined_byte_structure = FeagiByteStructure::create_from_2_existing(
        &json_byte_structure, 
        &neuron_byte_structure
    ).unwrap();

    // Verify the combined structure is a multi-struct
    assert!(combined_byte_structure.is_multistruct().unwrap());
    assert_eq!(combined_byte_structure.try_get_structure_type().unwrap(), FeagiByteStructureType::MultiStructHolder);
    assert_eq!(combined_byte_structure.contained_structure_count().unwrap(), 2);

    // Check the order of internal structure types
    let ordered_types = combined_byte_structure.get_ordered_object_types().unwrap();
    assert_eq!(ordered_types.len(), 2);
    assert_eq!(ordered_types[0], FeagiByteStructureType::JSON);
    assert_eq!(ordered_types[1], FeagiByteStructureType::NeuronCategoricalXYZP);

    // Serialize to bytes (simulate network transmission)
    let serialized_bytes = combined_byte_structure.copy_out_as_byte_vector();

    // Deserialize from bytes
    let received_combined_structure = FeagiByteStructure::create_from_bytes(serialized_bytes).unwrap();

    // Verify the received structure is still a multi-struct with correct properties
    assert!(received_combined_structure.is_multistruct().unwrap());
    assert_eq!(received_combined_structure.contained_structure_count().unwrap(), 2);

    // Extract individual structures from the multi-struct
    let received_json_structure_bytes = received_combined_structure.copy_out_single_byte_structure_from_multistruct(0).unwrap();
    let received_neuron_structure_bytes = received_combined_structure.copy_out_single_byte_structure_from_multistruct(1).unwrap();

    // Verify individual structure types are correct
    assert_eq!(received_json_structure_bytes.try_get_structure_type().unwrap(), FeagiByteStructureType::JSON);
    assert_eq!(received_neuron_structure_bytes.try_get_structure_type().unwrap(), FeagiByteStructureType::NeuronCategoricalXYZP);

    // Convert back to original data types
    let recovered_json_structure = JsonStructure::new_from_feagi_byte_structure(&received_json_structure_bytes).unwrap();
    let recovered_neuron_mappings = CorticalMappedXYZPNeuronData::new_from_feagi_byte_structure(&received_neuron_structure_bytes).unwrap();

    // Verify JSON data integrity
    let recovered_json_value = recovered_json_structure.borrow_json_value();
    assert_eq!(recovered_json_value, &json_data);

    // Verify neuron data integrity
    assert_eq!(recovered_neuron_mappings.get_number_contained_areas(), 2);
    assert!(recovered_neuron_mappings.contains(CorticalID::from_str("AAAAAA").unwrap()));
    assert!(recovered_neuron_mappings.contains(CorticalID::from_str("BBBBBB").unwrap()));

    let recovered_neurons_a = recovered_neuron_mappings.borrow(&CorticalID::from_str("AAAAAA").unwrap()).unwrap();
    let recovered_neurons_b = recovered_neuron_mappings.borrow(&CorticalID::from_str("BBBBBB").unwrap()).unwrap();

    let recovered_neuron_vec_a = recovered_neurons_a.copy_as_neuron_xyzp_vec();
    let recovered_neuron_vec_b = recovered_neurons_b.copy_as_neuron_xyzp_vec();

    assert_eq!(recovered_neuron_vec_a.len(), 2);
    assert_eq!(recovered_neuron_vec_b.len(), 1);
    assert_eq!(recovered_neuron_vec_a[0], neuron_a_1);
    assert_eq!(recovered_neuron_vec_a[1], neuron_a_2);
    assert_eq!(recovered_neuron_vec_b[0], neuron_b_1);

    println!("✓ Successfully combined, serialized, and deserialized JSON + Neuron data!");
}

#[test]
fn test_multistruct_with_multiple_json_and_neuron_structures() {
    // Create multiple JSON structures
    let json1 = JsonStructure::from_json_value(json!({"type": "config", "value": 1}));
    let json2 = JsonStructure::from_json_value(json!({"type": "metadata", "value": 2}));

    // Create multiple neuron structures
    let cortical_id_1 = CorticalID::from_str("TEST01").unwrap();
    let neuron_1 = NeuronXYZP::new(1, 1, 1, 0.1);
    let mut neurons_1 = NeuronXYZPArrays::new(1).unwrap();
    neurons_1.add_neuron(&neuron_1);
    let mut neuron_mappings_1 = CorticalMappedXYZPNeuronData::new();
    neuron_mappings_1.insert(cortical_id_1, neurons_1);

    let cortical_id_2 = CorticalID::from_str("TEST02").unwrap();
    let neuron_2 = NeuronXYZP::new(2, 2, 2, 0.2);
    let mut neurons_2 = NeuronXYZPArrays::new(1).unwrap();
    neurons_2.add_neuron(&neuron_2);
    let mut neuron_mappings_2 = CorticalMappedXYZPNeuronData::new();
    neuron_mappings_2.insert(cortical_id_2, neurons_2);

    // Convert to byte structures
    let json1_bytes = json1.as_new_feagi_byte_structure().unwrap();
    let json2_bytes = json2.as_new_feagi_byte_structure().unwrap();
    let neuron1_bytes = neuron_mappings_1.as_new_feagi_byte_structure().unwrap();
    let neuron2_bytes = neuron_mappings_2.as_new_feagi_byte_structure().unwrap();

    // Create multi-struct from all 4 structures
    let all_structures = vec![&json1_bytes, &neuron1_bytes, &json2_bytes, &neuron2_bytes];
    let combined_structure = FeagiByteStructure::create_from_multiple_existing(all_structures).unwrap();

    // Verify multi-struct properties
    assert!(combined_structure.is_multistruct().unwrap());
    assert_eq!(combined_structure.contained_structure_count().unwrap(), 4);

    // Verify structure order
    let ordered_types = combined_structure.get_ordered_object_types().unwrap();
    assert_eq!(ordered_types[0], FeagiByteStructureType::JSON);
    assert_eq!(ordered_types[1], FeagiByteStructureType::NeuronCategoricalXYZP);
    assert_eq!(ordered_types[2], FeagiByteStructureType::JSON);
    assert_eq!(ordered_types[3], FeagiByteStructureType::NeuronCategoricalXYZP);

    // Serialize and deserialize
    let bytes = combined_structure.copy_out_as_byte_vector();
    let received_structure = FeagiByteStructure::create_from_bytes(bytes).unwrap();

    // Extract and verify all structures
    for i in 0..4 {
        let extracted = received_structure.copy_out_single_byte_structure_from_multistruct(i).unwrap();
        match i {
            0 | 2 => assert_eq!(extracted.try_get_structure_type().unwrap(), FeagiByteStructureType::JSON),
            1 | 3 => assert_eq!(extracted.try_get_structure_type().unwrap(), FeagiByteStructureType::NeuronCategoricalXYZP),
            _ => unreachable!(),
        }
    }

    println!("✓ Successfully handled multi-struct with 4 different structures!");
} 