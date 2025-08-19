## Encoders and Decoders
Coders are structs that handle translation between neural data and standard computer data. These structs are used
internally within this crate, there is no need to instantiate them outside of it, as such the details of their implementation is important mainly to contributors of FEAGI itself.

**Encoders** convert computer data into neuron activity.

**Decoders** convert neuron activity into computer data