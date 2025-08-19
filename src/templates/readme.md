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


robot 
- rgb camera
- 2 rotary motors
- 3 proximity sensors

[what we have]
(init)
register_sensor(camera, cortical grouping index 0, 1 channel, expected dimensions of image)
register_sensor(proximity, coritcal grouping index 0, 3 channels, (1,1,20) as dimensions given 20 is resolution neurons)
register_motor(rotary_motor, cortical_grouping_index_0, 2 channels, (1,1,20) as dimensions given 20 is resolution neurons)

register_sensor_device_channel(camera, cortical_grouping_index 0, channel 0, processing_steps[resize + crop])
register_sensor_device_channel(proximity, cortical_grouping_index 0, channel 0, processing_steps[rolling window of length 5])
register_sensor_device_channel(proximity, cortical_grouping_index 0, channel 1, processing_steps[rolling window of length 5])
register_sensor_device_channel(proximity, cortical_grouping_index 0, channel 2, processing_steps[rolling window of length 5])

register_motor_device_channel(rotary_motor, cortical_grouping_index 0, channel 0, processing_steps[rolling window of length 3)
register_motor_device_channel(rotary_motor, cortical_grouping_index 0, channel 1, processing_steps[rolling window of length 3)


sending
send_sensor_data(proximity, cortical_grouping_index 0, channel 1, recieved input 0.5)
send()


// what we should do

we a registering 3 proximity sensors, with the same resolution, under the same group (cortical area), but they want
the middle proximity sensor in this case to have a smoother average.

register_float_sensor(proximity, coritcal grouping index 0, resolution 20,
[
processing_steps[rolling window of length 5],
processing_steps[rolling window of length 10],
processing_steps[rolling window of length 5],
])

send_sensor_data(proximity, cortical_grouping_index 0, channel 1, recieved input 0.5)


we a registering 3 proximity sensors, with the same resolution, but different groups (cortical area), but they want
the middle proximity sensor in this case to have a smoother average.

register_float_sensor(proximity, coritcal grouping index 0, resolution 20,
[
processing_steps[rolling window of length 5],
])
register_float_sensor(proximity, coritcal grouping index 1, resolution 20,
[
processing_steps[rolling window of length 5],
])
register_float_sensor(proximity, coritcal grouping index 2, resolution 20,
[
processing_steps[rolling window of length 5],
])


we a registering 3 proximity sensors, with different resolution, but different groups (cortical area), but they want
the middle proximity sensor in this case to have a smoother average.

register_float_sensor(proximity, coritcal grouping index 0, resolution 20,
[
processing_steps[rolling window of length 5],
])
register_float_sensor(proximity, coritcal grouping index 1, resolution 40,
[
processing_steps[rolling window of length 3, nonlinear rolling window of length 2],
])
register_float_sensor(proximity, coritcal grouping index 2, resolution 20,
[
processing_steps[rolling window of length 5],
])






sensor::proxmity::register_group(cortical_grouping_index 0, resolution 20, number_of_channels 1)      // {initialize default identity processor}
sensor::proximity::set_data_processing(cortical_group 0, channel 0, [rolling window of length 5])


set_sensor_processor(proximity, group 0, [rolling window of length 5])

register.sensor.proximity_group(group_index 0, sensor_resolution 20)

rsgister.sensor.image_camera()


# Rules -  what is allowed without reinitializing

## things that require connector reinitialization
- anything that adds / removes a cortical area
- anything that changes a dimension of a cortical area
  - resolution changes
- these are things that should be part of encoder definitions

## things that dont
- these things should be only managed by provessors, never encoders (which require reinits to change)
- processing steps
- segmention of images
  - modulation, eccentricity
- thresholds
- 






