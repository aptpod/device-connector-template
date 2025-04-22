mod hello;
mod hexdump;

// Implement plugin interface.
dc_core::define_plugin!(
    "{{crate_name}}";
    hello::HelloSrcElement,
    hexdump::HexdumpSinkElement,
);
