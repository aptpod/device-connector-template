mod hello;
mod hexdump;

// Implement plugin interface.
device_connector::define_dc_load!(hello::HelloSrcElement, hexdump::HexdumpSinkElement,);
