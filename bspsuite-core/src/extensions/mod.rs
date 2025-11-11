use anyhow::Result;
use libloading::Library;

mod loader;
use loader::{UnsafeSymbol, get_unsafe_symbol};

pub use loader::{find_extensions, load_extensions};

/// Extension interface version that we expect extensions to present.
/// If a call to bspsuite_ext_get_interface_version returns a version
/// that does not match this value, the extension will not be loaded.
pub const BSPSUITE_EXT_INTERFACE_CURRENT_VERSION: usize = 1;

const BSPSUITE_EXT_SYM_GETINTERFACEVERSION: &[u8] = b"bspsuite_ext_get_interface_version";
type ExtFnGetInterfaceVersion = extern "C" fn() -> usize;

const BSPSUITE_EXT_SYM_PRESENT_SERVICES: &[u8] = b"bspsuite_ext_present_services";
type ExtFnPresentServices = extern "C" fn(&ExtensionServicesApi) -> ExtensionServicesResult;

pub struct Extension
{
	library: Library,
	present_services_symbol: UnsafeSymbol<ExtFnPresentServices>,
}

impl Extension
{
	pub fn from(library: Library) -> Result<Extension>
	{
		// Unsafe symbol calls are allowed here, since all the symbols are stored
		// privately on the struct, and the struct cannot live longer than the library.
		let present_servives_symbol: UnsafeSymbol<ExtFnPresentServices> =
			unsafe { get_unsafe_symbol(&library, BSPSUITE_EXT_SYM_PRESENT_SERVICES)? };

		return Ok(Extension {
			library: library,
			present_services_symbol: present_servives_symbol,
		});
	}

	pub fn present_services(&self, api: &ExtensionServicesApi) -> ExtensionServicesResult
	{
		return (*self.present_services_symbol)(api);
	}
}

#[repr(C)]
/// Result codes that may be returned by an extension from a call to
/// bspsuite_ext_present_services.
pub enum ExtensionServicesResult
{
	/// The extension was able to obtain pointers to all the services it
	/// required.
	Ok,

	/// The extension was unable to obtain pointers to one or more services that
	/// it required.
	Missed,
}

#[repr(C)]
pub struct ExtensionServicesApi {}

impl ExtensionServicesApi
{
	pub fn temp(&self) -> i32
	{
		return 1234;
	}
}
