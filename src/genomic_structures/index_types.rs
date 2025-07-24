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


// 0-255 in hexadecimal. cortical ID index for a specific cortical type in a genome
create_type_1d!(CorticalGroupingIndex, u8);

// Local Connector side mapping for a device to a CorticalGroupingIndex
create_type_1d!(AgentDeviceIndex, u32);

// Nested Mapping to a specific channel within a IO cortical area
create_type_1d!(CorticalIOChannelIndex, u32);