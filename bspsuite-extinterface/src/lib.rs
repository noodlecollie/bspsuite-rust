/// Extension interface version that we expect extensions to return.
/// If a call to bspsuite_ext_get_interface_version returns a version
/// that does not match this value, the extension will not be loaded.
pub const EXTENSION_INTERFACE_VERSION: usize = 1;

/// Symbol name for the bspsuite_ext_get_interface_version function.
pub const SYMBOL_GET_INTERFACE_VERSION: &[u8] = b"bspsuite_ext_get_interface_version";

/// Function signature for the bspsuite_ext_get_interface_version function.
/// The extension implementing this function should return the value of
/// EXTENSION_INTERFACE_VERSION.
pub type ExtFnGetInterfaceVersion = extern "C" fn() -> usize;
