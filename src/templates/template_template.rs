// The following are format templates that are used by other templates. This file generally does
// not need modification

#[macro_export]
macro_rules! iopu_definition {
    () =>
    {
        $cortical_io_type_enum_name:ident {
            $(
                $cortical_type_key_name:ident => {
                    friendly_name: $display_name:expr,
                    base_ascii: $base_ascii:expr,
                    channel_dimensions: $channel_dimensions:expr,
                    io_variants: $io_variants:expr,
                    encoder_type: $encoder_type:expr,
                }
            ),* $(,)?
        }
    }
}