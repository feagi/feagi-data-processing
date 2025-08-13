# Sensor / Motor Documentation
These Macros are essentially template definitions for all sensor and motor types that are supported by FEAGI, and is used as the backbone for downstream sensor and motor code generation for use in connector and FEAGI itself.

## Sensors
















# Discussion
(These items are WIP and are subject to change)

- How do we handle multi-axis directional sensors (accelerometers, gyroscope)?
  - Should we have 2 types? One for Euler one foe Quaternion?
    - THe problem with this is euler feels easier to use, but is impacted by gimbal lock? 
    - Other solution is to use quaternion internally only but be able to process in eulers * ?
- How should we handle pure rotation sensors (gyroscope) vs vector (accelerometer) ?
- how should we manage the neural encoding?

misc -> take in and output a 3d vector as user can do anything
motor needs to exist
exposure to python -> Device ID

brain output
- relative vs absolute
- cortical ID format
  - used to be o__mot
  - new way overall
    - o (output signifier)
    - & & & (brain output identifier)
      - single color simple led -> led
      - display 2d output -> d2d
      - display 3d (point cloud) -> d3d
      - 
    - 0 0 (cortical group)
  - rotory motor (r / a)
    - ormr00 (output rotary motor relative cortical group 00)
    - orma00 (output rotary motor absolute cortical group 00)
  - servo motor (r / a)
  - 