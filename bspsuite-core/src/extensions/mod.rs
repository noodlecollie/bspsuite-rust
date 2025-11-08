use std::path::Path;

use libloading::{Error, Library, Symbol, library_filename};

const BSPSUITE_EXT_INTERFACE_CURRENT_VERSION: usize = 1;
const BSPSUITE_EXT_SYM_GETINTERFACEVERSION: &[u8] = b"bspsuite_ext_get_interface_version";
type ExtFnGetInterfaceVersion = fn() -> usize;

pub fn load_extension_library(path: &Path) -> Result<libloading::Library, String>
{
	let load_result: Result<Library, Error> =
		unsafe { Library::new(library_filename(path.as_os_str())) };

	let library: Library = load_result.or_else(|err| {
		Err(format!(
			"Could not load extension {}. {}",
			path.to_str().unwrap(),
			err.to_string()
		))
	})?;

	let lookup_result: Result<Symbol<ExtFnGetInterfaceVersion>, Error> =
		unsafe { library.get(BSPSUITE_EXT_SYM_GETINTERFACEVERSION) };

	let get_interface_version: Symbol<ExtFnGetInterfaceVersion> = lookup_result.or_else(|err| {
		Err(format!(
			"Could not load extension {}. {}",
			path.to_str().unwrap(),
			err.to_string()
		))
	})?;

	let interface_version: usize = get_interface_version();

	if interface_version != BSPSUITE_EXT_INTERFACE_CURRENT_VERSION
	{
		return Err(format!(
			"Could not load extension {}. Required interface version \
			{BSPSUITE_EXT_INTERFACE_CURRENT_VERSION}, but extension provided \
			interface version {interface_version}.",
			path.to_str().unwrap()
		));
	}

	return Ok(library);
}
