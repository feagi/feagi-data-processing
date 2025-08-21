use crate::FeagiDataError;

//region Macros
/// Defines the index of something as an integer of a certain type
macro_rules! define_index {
    ($name:ident, $inner:ty, $doc:expr) => {
        #[doc = $doc]
        #[repr(transparent)]
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord
        )]
        pub struct $name(pub $inner);

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                $name(value)
            }
        }

        impl From<$name> for $inner {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

/// Define the count of something that cannot be 0
macro_rules! define_nonzero_count {
    ($name:ident, $base:ty, $doc:expr) => {
        #[doc = $doc]
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(pub $base);
        
        impl $name {
            /// Creates a new instance, returns Err if validation fails
            pub fn new(value: $base) -> Result<Self, FeagiDataError> {
                if value == 0 {
                    return Err(FeagiDataError::BadParameter("Count cannot be zero!".into()));
                }
                Ok($name(value))
            }
        }
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
        
    }
}
//endregion


define_index!(CorticalGroupIndex, u8, 
    "Index for grouping cortical areas of the same type within a genome.

This index distinguishes between multiple instances of the same cortical type.
For example, multiple vision sensors would have different CorticalGroupingIndex
values (0, 1, 2, etc.) while sharing the same base cortical type.

# Range
Values are limited to 0-255 (u8) and are encoded in hexadecimal within cortical IDs.
This provides support for up to 256 instances of each cortical type.

# Usage in Cortical IDs
The index appears as the last two characters of a cortical ID:
- \"ivis00\" = Vision sensor, grouping index 0
- \"ivis01\" = Vision sensor, grouping index 1
- \"omot0A\" = Motor output, grouping index 10 (hexadecimal A)"
);

// NOTE: We do not need a Cortical Group Count!

define_index!(CorticalChannelIndex, u32,
    "Index for addressing specific channels within an I/O cortical area.

Cortical areas can contain multiple channels for processing different
aspects of data. This index addresses individual channels within a
specific cortical area for fine-grained data routing."
);

define_nonzero_count!(CorticalChannelCount, u32, "The number of Cortical Channels. Cannot be zero."
);