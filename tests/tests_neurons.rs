use feagi_core_data_structures_and_processing::neuron_data::neurons::NeuronXYZP;
use feagi_core_data_structures_and_processing::neuron_data::neuron_arrays::NeuronXYZPArrays;
use feagi_core_data_structures_and_processing::cortical_data::CorticalID;
use feagi_core_data_structures_and_processing::neuron_data::neuron_mappings::CorticalMappedXYZPNeuronData;
use feagi_core_data_structures_and_processing::byte_structures::feagi_byte_structure;
use feagi_core_data_structures_and_processing::byte_structures::feagi_byte_structure::FeagiByteStructureCompatible;

#[test]
fn test_serialize_deserialize_neuron_mapped_areas() {
    
    // cortical area A
    let cortical_id_a = CorticalID::from_str("AAAAAA").unwrap();
    let neuron_a_1 = NeuronXYZP::new(1, 2, 3, 0.5);
    let neuron_a_2 = NeuronXYZP::new(4, 5, 7, 0.2);
    let mut neurons_a = NeuronXYZPArrays::new(2).unwrap(); // lets preallocate
    neurons_a.add_neuron(&neuron_a_1);
    neurons_a.add_neuron(&neuron_a_2);
    
    
    // cortical area b
    let cortical_id_b = CorticalID::from_str("BBBBBB").unwrap();
    let neuron_b_1 = NeuronXYZP::new(8, 9, 10, 0.5);
    let neuron_b_2 = NeuronXYZP::new(11, 12, 13, 0.2);
    let mut neurons_b = NeuronXYZPArrays::new(1).unwrap(); // incorrect preallocation (system should grow)
    neurons_b.add_neuron(&neuron_b_1);
    neurons_b.add_neuron(&neuron_b_2);

    assert_eq!(
        neurons_a.get_number_of_neurons_used(),
        neurons_b.get_number_of_neurons_used()
    );
    
    // cortical mappings
    let mut cortical_mappings = CorticalMappedXYZPNeuronData::new();
    cortical_mappings.insert(cortical_id_a, neurons_a);
    cortical_mappings.insert(cortical_id_b, neurons_b);

    // byte data serialization
    let byte_structure = cortical_mappings.as_new_feagi_byte_structure().unwrap();
}

