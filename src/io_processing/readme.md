# IO Processing
This module is used extensively by FEAGI Connector to efficiently cache and process input and output data to and from FEAGI.

## Byte Structure
As we transmit various types of data, from authentication, command and control, and neuron data, we have a custom byte structure to facilitate that, which is handled by the FEAGI byte Structure struct, which represents a single frame of data. This data may be nested (up to 1 level), and can represent various types of data. Users do not need to interface with this directly.

Various data structures can implement the trait "FeagiByteStructureCompatible" to ensure that they can be serialized / deserialized in the Feagi Byte Structure Wrapper.

To see technical information on the specifications of the Feagi Byte Structure, please read here (TODO).

## Stream Cache Processor
As neuronal data comes in (or before it goes out), the user may want to define a filter of some sort ot act upon the coded data. This are handled by Stream Cache Processors, which are configurable structures that act upon some processing method onto receiving / sending data. There are various types, depending on the type of data encoded:
- Identity
  - Does nothing. No filtering, averaging, anything.
- Sliding Window Average
  - Caches the previous few data points, and applies a sliding average window to it to stabilize the data at the cost of temporal resolution

Stream Cache Processors are defined per channel.

## Internal Cache Elements
These structs are not user exposed but at useful to understand how data is processed and cached

### Device Group Cache
Holds all cache and structure information for a given cortical type, whether there is only a single 2 channel cortical area for it, or 50.

### Call Back Manager
In the case of motors, users may want to "subscribe" a certain function to process new motor input immediately as it arrives. The CallBackManager struct handles registering and making these calls.

