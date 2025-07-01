# Cortical IDs
Cortical IDs are 6 ASCII characters long (composed of alphanumeric (upper and lowercase) and underscore characters only) genome unique identifiers to cortical areas within FEAGI.

They are not fully random, however, and have some rules

### The first character denotes the type of cortical area
Only the following are allowed first characters:
- "c": Custom cortical area - handles processing within the brain from input to output
- "m": Memory cortical area - handles recall of information
- "i": Input cortical area - represents data input from a sensor
- "o": Output cortical area - represents data output to a motor
- "_": Core cortical area - related to base FEAGI functionality, part of every genome

## Rules and Definitions per Cortical Area Type

### Custom Cortical Area
- Following the first "c" character, all 5 other characters may be any alphanumeric character (either uppercase or lowercase) with no further logic

### Memory Cortical Area
- Following the first "m" character, all 5 other characters may be any alphanumeric character (either uppercase or lowercase) with no further logic

### Core Cortical Area
These are static cortical areas. The following cortical areas will exist on all genomes:
- ___pwr: Is always on as long as the genome is alive

### Input Cortical Area
Following the "i", the next 3 characters refer to the sensor type, and the last 2 are hexadecimal (0-f) characters representing the "index" of that sensor (range from 0-255). The following sensor types area available (posted here with the example index of 0):
- **iinf00**
  - Infrared Sensor
- **iiif00**
  - Reverse Infrared Sensor
- **ipro00**
  - Proximity Sensor
- **igpd00**
  - Digital GPIO input
- **igpa00**
  - Analog GPIO Input
- **iacc00**
  - Accelerometer Input
- **igyr00**
  - Gyro Input
- **ieul00**
  - Euler Input
- **isho00**
  - Shock Input
- **ibat00**
  - Battery Input
- **icom00**
  - Compass Input
- **ivcc00**
  - Center Vision Input (Grayscale)
- **ivtl00**
  - Top Left Vision Input (Grayscale)
- **ivtm00**
  - Top Middle Vision Input (Grayscale)
- **ivtr00**
  - Top Right Vision Input (Grayscale)
- **ivml00**
  - Middle Left Vision Input (Grayscale)
- **ivmr00**
  - Middle Right Vision Input (Grayscale)
- **ivbl00**
  - Bottom Left Vision Input (Grayscale)
- **ivbm00**
  - Bottom Middle Input (Grayscale)
- **ivbr00**
  - Bottom Right Vision Input (Grayscale)
- **iVcc00**
  - Center Vision Input (Color)
- **iVtl00**
  - Top Left Vision Input (Color)
- **iVtm00**
  - Top Middle Vision Input (Color)
- **iVtr00**
  - Top Right Vision Input (Color)
- **iVml00**
  - Middle Left Vision Input (Color)
- **iVmr00**
  - Middle Right Vision Input (Color)
- **iVbl00**
  - Bottom Left Vision Input (Color)
- **iVbm00**
  - Bottom Middle Input (Color)
- **iVbr00**
  - Bottom Right Vision Input (Color)
- **imis00**
  - Miscellaneous
- **ispo00**
  - Servo Position
- **ismo00**
  - Servo Motion
- **iidt00**
  - ID Trainer
- **ipre00**
  - Pressure
- **ilid00**
  - Lidar
- **iear00**
  - Audio


### Output Cortical Area
Following the "0", the next 3 characters refer to the motor type, and the last 2 are hexadecimal (0-f) characters representing the "index" of that motor (range from 0-255). The following motor types area available (posted here with the example index of 0):
- **omot00**
    - Motor (simple spinning motor)
- **ospo00**
    - Servo Position
- **osmo00**
    - Servo Motion
- **omcl00**
    - Motion Control
- **pbat00**
    - Battery
