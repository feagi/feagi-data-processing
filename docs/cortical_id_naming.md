# Cortical ID naming conventions 

6 letter id
first letter from left: i (IPU), o (OPU), m (memory), c(custom), _(core)
next three letters are for functional abbreviations: e.g. ic4 (image camera 4th segment. aka. central vision)
last two letters are reserved for group index. 

FEAGI primarily uses 'rate encoding' method to encode sensory data into neuronal activities.



# IPU (Input Processing Unit)

## Suppoted Data Types:
### Scalars
### Vectors
### Percentages
### Quaternions




### Image Camera
- [ ] iic0xx - Bottom Left of the 9 segment vision
- [ ] iic1xx - Bottom Middle of the 9 segment vision
- [ ] iic2xx - Bottom Right of the 9 segment vision
- [ ] iic3xx - Middle Left of the 9 segment vision
- [ ] iic4xx - Middle Middle of the 9 segment vision aka. central vision
- [ ] iic5xx - Middle Right of the 9 segment vision
- [ ] iic6xx - Top Left of the 9 segment vision
- [ ] iic7xx - Top Middle of the 9 segment vision
- [ ] iic8xx - Top Right of the 9 segment vision

 Data type: uint8
 Encoding: Rate + Magnitude
 
 


### IMU (Inertial Measurement Unit)

- [ ] imuq - IMU represented as a quaternion  
 Channel width: 9 (3 for Accelerometer, 3 for Gyroscope, 3 for magnetometer)
 Data type: Quatenion


### Infraret

### Proximity

### Lidar

### Pointer Device

### Battery Gauge

### BCI (Brain Computer Interface)

### Object Classifier


### Audio


### GPIO

Encoding: Rate + Magnitude + Label


### Servo Motor Encoder


### Rotary Motor Encoder


### Miscelaneous 




# OPU (Oputput Processing Unit)

* The forth position in the cortical id of select OPUs will be either 'a','i', or '_'. 'a' stands for 'absolute' targeting and 'i' stands for 'incremental'. In absolute 


## Suppoted Data Types:
### Scalars
### Vectors
### Percentages
### Quaternions



##Cortical Types:
### Rotary Motor

### Servo Motor

### GPIO

### Object Classifier

### Visual Insight

### Sound

### Movement


### Miscelaneous 

### Pointer





