# Cortical ID naming conventions 

6 letter id
first letter from left: i (IPU), o (OPU), m (memory), c(custom), _(core)
next three letters are for functional abbreviations: e.g. ic4 (image camera 4th segment. aka. central vision)
last two letters are reserved for group index. 



# IPU (Input Processing Unit)

## Suppoted Data Types:
### Scalars
### Vectors
### Percentages
### Quaternions




### Image Camera
- [ ] iic0 - Bottom Left of the 9 segment vision
- [ ] iic1 - Bottom Middle of the 9 segment vision
- [ ] iic2 - Bottom Right of the 9 segment vision
- [ ] iic3 - Middle Left of the 9 segment vision
- [ ] iic4 - Middle Middle of the 9 segment vision aka. central vision
- [ ] iic5 - Middle Right of the 9 segment vision
- [ ] iic6 - Top Left of the 9 segment vision
- [ ] iic7 - Top Middle of the 9 segment vision
- [ ] iic8 - Top Right of the 9 segment vision

 Data type: uint8


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





