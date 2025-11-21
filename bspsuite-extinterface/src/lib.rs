// Code architecture of this module is informed by
// https://users.rust-lang.org/t/linking-issues-when-designing-a-dynamic-plugin-based-architecture/136388

pub mod dummy_api;
pub mod probe_api;

/// Struct whose sole responsibility is to expose a versioned entry point API to
/// users of an extension.
#[repr(C)]
pub struct ExtensionInfo
{
	pub probe_api_version: usize,
	pub probe_fn: probe_api::ExtFnProbe,
}

/// Name of the library symbol that exposes the extension's interface
/// information.
pub const SYMBOL_EXTENSION_INFO: &[u8] = b"bspsuite_ext_info";

#[macro_export]
macro_rules! implement_extension_info {
	($probe:expr) => {
		#[doc(hidden)]
		#[allow(non_upper_case_globals)]
		#[unsafe(no_mangle)]
		pub static bspsuite_ext_info: $crate::ExtensionInfo = $crate::ExtensionInfo {
			probe_api_version: $crate::probe_api::API_VERSION,
			probe_fn: $probe,
		};
	};
}
