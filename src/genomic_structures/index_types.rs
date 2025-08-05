//! Type-safe index wrappers for FEAGI genomic structures.
//!
//! This module provides strongly-typed wrappers around primitive numeric types
//! to create distinct index types for different aspects of FEAGI.

/// Macro to create type-safe 1D index wrappers.
///
/// This macro generates a new type that wraps a primitive numeric type,
/// providing type safety while maintaining easy conversion and dereferencing.
/// Each generated type implements common traits for arithmetic, comparison,
/// and display operations.
///
/// # Generated Implementations
/// - `From<$base>` and `From<$name>` for easy conversion
/// - `Deref` for transparent access to the underlying value
/// - `Display` for formatted output
/// - Standard comparison and hashing traits
macro_rules! create_type_1d {
    ($name:ident, $base:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub $base);

        impl From<$base> for $name {
            fn from(value: $base) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $base {
            fn from(value: $name) -> $base {
                value.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = $base;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

/// Index for grouping cortical areas of the same type within a genome.
///
/// This index distinguishes between multiple instances of the same cortical type.
/// For example, multiple vision sensors would have different CorticalGroupingIndex
/// values (0, 1, 2, etc.) while sharing the same base cortical type.
///
/// # Range
/// Values are limited to 0-255 (u8) and are encoded in hexadecimal within cortical IDs.
/// This provides support for up to 256 instances of each cortical type.
///
/// # Usage in Cortical IDs
/// The index appears as the last two characters of a cortical ID:
/// - "ivis00" = Vision sensor, grouping index 0
/// - "ivis01" = Vision sensor, grouping index 1
/// - "omot0A" = Motor output, grouping index 10 (hexadecimal A)
create_type_1d!(CorticalGroupingIndex, u8);

/// Optional undex mapping local device instances to cortical grouping indices.
///
/// This provides a mapping layer between physical devices (cameras, motors, etc.)
/// connected to an agent and their corresponding cortical areas in the genome.
/// Allows for flexible device-to-cortical-area assignments.
///
/// # Use Case
/// If you have multiple sensors of the same type across various cortical areas, using
/// Agent Device Mapping indexes can be a uniform way to address them all
create_type_1d!(AgentDeviceIndex, u32);

/// Index for addressing specific channels within an I/O cortical area.
///
/// Cortical areas can contain multiple channels for processing different
/// aspects of data. This index addresses individual channels within a
/// specific cortical area for fine-grained data routing.
///
/// # Examples
/// - A multi-axis motor area might have separate channels for each axis
create_type_1d!(CorticalIOChannelIndex, u32);