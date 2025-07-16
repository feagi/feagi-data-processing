//! Core neuron representation using XYZP coordinates.
//!
//! This module defines the fundamental [`NeuronXYZP`] structure that represents
//! individual neurons in the FEAGI system using a 4-dimensional coordinate system:
//! X, Y, Z spatial coordinates and P (potential) value.
//!
//! # XYZP Coordinate System
//!
//! The XYZP system is the primary neuron addressing scheme in FEAGI:
//! - **X, Y, Z**: 3D spatial coordinates within a cortical area
//! - **P**: Neuron potential/activation value (typically 0.0 to 1.0)
//!
//! # Usage
//!
//! ```rust
//! use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZP;
//!
//! // Create a neuron at coordinates (10, 5, 2) with potential 0.8
//! let neuron = NeuronXYZP::new(10, 5, 2, 0.8);
//!
//! // Access coordinates
//! let (x, y, z, p) = neuron.as_tuple();
//! println!("Neuron at ({}, {}, {}) with potential {}", x, y, z, p);
//! ```
//!
//! # Design Notes
//!
//! - Coordinates use `u32` for maximum addressable space (4 billion neurons per dimension)
//! - Potential uses `f32` for floating-point precision in activation values
//! - Structure is `Clone`, `Debug`, and `PartialEq` for ease of use in collections
//! - Optimized for memory efficiency and fast copying

/// Represents a single neuron using XYZP coordinate system.
///
/// This is the fundamental neuron representation in FEAGI, combining 3D spatial
/// coordinates with an activation potential value. Each neuron exists at a specific
/// location within a cortical area and has an associated potential value.
///
/// # Fields
///
/// - `x`: X-coordinate within the cortical area (0 to 4,294,967,295)
/// - `y`: Y-coordinate within the cortical area (0 to 4,294,967,295)  
/// - `z`: Z-coordinate within the cortical area (0 to 4,294,967,295)
/// - `p`: Potential/activation value (typically 0.0 to 1.0, but not strictly limited)
///
/// # Examples
///
/// ```rust
/// use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZP;
///
/// // Create a neuron with specific coordinates and potential
/// let neuron = NeuronXYZP::new(100, 50, 25, 0.75);
///
/// // Access individual fields
/// assert_eq!(neuron.x, 100);
/// assert_eq!(neuron.y, 50);
/// assert_eq!(neuron.z, 25);
/// assert_eq!(neuron.p, 0.75);
///
/// // Get as tuple for pattern matching
/// match neuron.as_tuple() {
///     (x, y, z, p) if p > 0.5 => println!("Highly active neuron at ({}, {}, {})", x, y, z),
///     _ => println!("Low activity neuron"),
/// }
/// ```
///
/// # Memory Layout
///
/// The structure is optimized for memory efficiency:
/// - Total size: 16 bytes (3 × 4 bytes for coordinates + 4 bytes for potential)
/// - No padding due to uniform 4-byte field alignment
/// - Suitable for large arrays and high-performance processing
#[derive(Clone, Debug, PartialEq)]
pub struct NeuronXYZP {
    /// X-coordinate within the cortical area.
    pub x: u32,
    /// Y-coordinate within the cortical area.
    pub y: u32,
    /// Z-coordinate within the cortical area.
    pub z: u32,
    /// Neuron potential/activation value of the neuron. Can be positive or negative, unbounded
    pub p: f32
}

impl NeuronXYZP {
    /// Creates a new neuron with the specified coordinates and potential.
    ///
    /// # Arguments
    ///
    /// * `x` - X-coordinate within the cortical area
    /// * `y` - Y-coordinate within the cortical area  
    /// * `z` - Z-coordinate within the cortical area
    /// * `p` - Neuron potential/activation value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZP;
    ///
    /// // Create a neuron at the origin with no activation
    /// let inactive_neuron = NeuronXYZP::new(0, 0, 0, 0.0);
    ///
    /// // Create an active neuron at a specific location
    /// let active_neuron = NeuronXYZP::new(100, 200, 50, 0.85);
    /// ```
    pub fn new(x: u32, y: u32, z: u32, p: f32) -> Self {
        NeuronXYZP { x, y, z, p }
    }
    
    /// Returns the neuron's coordinates and potential as a tuple.
    ///
    /// This method provides a convenient way to destructure the neuron's
    /// data for pattern matching or function arguments that expect tuples.
    ///
    /// # Returns
    ///
    /// A tuple `(x, y, z, p)` containing the neuron's coordinates and potential.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use feagi_core_data_structures_and_processing::neuron_data::xyzp::NeuronXYZP;
    ///
    /// let neuron = NeuronXYZP::new(10, 20, 30, 0.5);
    /// let (x, y, z, potential) = neuron.as_tuple();
    ///
    /// assert_eq!(x, 10);
    /// assert_eq!(y, 20);
    /// assert_eq!(z, 30);
    /// assert_eq!(potential, 0.5);
    ///
    /// // Useful for pattern matching
    /// match neuron.as_tuple() {
    ///     (_, _, _, p) if p > 0.8 => println!("Highly active neuron"),
    ///     (_, _, _, p) if p > 0.3 => println!("Moderately active neuron"),
    ///     _ => println!("Low activity neuron"),
    /// }
    /// ```
    pub fn as_tuple(&self) -> (u32, u32, u32, f32) {
        (self.x, self.y, self.z, self.p)
    }
}