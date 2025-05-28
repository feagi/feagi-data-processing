# Vision Sensory Module

As vision is more complex than other sensor types due to the amount of processing required, a module is reserved for it.

The general goal of the vision module is to handle cases of receiving singular image frames, processing
them (resizing / cropping, color adjustments, etc), and then segmenting them into central and peripheral vision regions
before converting this structure into neuronal data that can be sent to FEAGI
