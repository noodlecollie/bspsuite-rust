use std::fs;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
#[cfg(target_os = "linux")]
use libloading::os::linux::Symbol as UnsafeSymbol;
#[cfg(target_os = "windows")]
use libloading::os::windows::Symbol as UnsafeSymbol;
use libloading::{Library, Symbol, library_filename};
use target_lexicon::{HOST, OperatingSystem};

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

pub fn find_extensions(root: &Path) -> Result<Vec<PathBuf>>
{
	let entries: ReadDir = fs::read_dir(root).with_context(|| {
		format!(
			"Could not read extensions from directory {}",
			root.to_str().unwrap()
		)
	})?;

	let file_ext: &str = library_extension_for_platform();
	let mut out_paths: Vec<PathBuf> = Vec::new();

	for entry in entries
	{
		if let Ok(entry) = entry
		{
			let path: PathBuf = entry.path();

			if let Some(ext) = path.extension()
				&& ext == file_ext
			{
				out_paths.push(path.clone());
			}
		}
	}

	return Ok(out_paths);
}

pub fn load_extension<'lib>(path: &PathBuf) -> Result<Extension>
{
	let library: Library = unsafe { Library::new(library_filename(path.as_os_str())) }?;
	check_extension_interface_version(&library)?;

	return Extension::from(library);
}

pub fn load_extensions(paths: &Vec<PathBuf>) -> Vec<Result<Extension>>
{
	return paths.iter().map(|path| load_extension(path)).collect();
}

const fn library_extension_for_platform() -> &'static str
{
	return match HOST.operating_system
	{
		OperatingSystem::Windows => ".dll",
		OperatingSystem::Linux => ".so",
		_ => panic!("Unsupported operating system"),
	};
}

fn check_extension_interface_version(library: &Library) -> Result<()>
{
	let get_interface_version: Symbol<ExtFnGetInterfaceVersion> =
		unsafe { library.get(BSPSUITE_EXT_SYM_GETINTERFACEVERSION) }?;

	let interface_version: usize = get_interface_version();

	if interface_version != BSPSUITE_EXT_INTERFACE_CURRENT_VERSION
	{
		bail!(
			"Required interface version {BSPSUITE_EXT_INTERFACE_CURRENT_VERSION}, \
			but extension provided interface version {interface_version}."
		);
	}

	Ok(())
}

// It is the caller's responsibility that the symbol is not used after the
// library is unloaded.
unsafe fn get_unsafe_symbol<T>(library: &Library, name: &[u8]) -> Result<UnsafeSymbol<T>>
{
	let symbol: Symbol<T> = unsafe { library.get(name) }?;
	return unsafe { Ok(symbol.into_raw()) };
}
