use std::fs;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use libloading::{Library, Symbol};
use log::{debug, trace};
use target_lexicon::{HOST, OperatingSystem};

use super::{BSPSUITE_EXT_SYM_GETINTERFACEVERSION, ExtFnGetInterfaceVersion, Extension};

#[cfg(target_os = "linux")]
pub use libloading::os::unix::Symbol as UnsafeSymbol;
#[cfg(target_os = "windows")]
pub use libloading::os::windows::Symbol as UnsafeSymbol;

// It is the caller's responsibility that the symbol is not used after the
// library is unloaded.
pub unsafe fn get_unsafe_symbol<T>(library: &Library, name: &[u8]) -> Result<UnsafeSymbol<T>>
{
	let symbol: Symbol<T> = unsafe { library.get(name) }?;
	return unsafe { Ok(symbol.into_raw()) };
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

pub fn load_extensions(paths: &Vec<PathBuf>, interface_version: usize) -> Vec<Result<Extension>>
{
	return paths
		.iter()
		.map(|path| {
			load_extension(path, interface_version).map_err(|err| {
				err.context(format!(
					"Failed to load extension {}",
					path.to_str().unwrap()
				))
			})
		})
		.collect();
}

fn load_extension<'lib>(path: &PathBuf, interface_version: usize) -> Result<Extension>
{
	let library: Library = unsafe { Library::new(path.as_os_str()) }?;

	let get_interface_version: Symbol<ExtFnGetInterfaceVersion> =
		unsafe { library.get(BSPSUITE_EXT_SYM_GETINTERFACEVERSION) }?;

	let received_version: usize = get_interface_version();

	trace!(
		"Extension {} reported interface version {received_version}",
		path.to_str().unwrap()
	);

	if received_version != interface_version
	{
		bail!(
			"Required interface version {interface_version}, \
			but extension provided interface version {received_version}."
		);
	}

	let name: String = compute_library_name(path.file_stem().unwrap().to_str().unwrap());
	let result: Result<Extension> = Extension::from(name, library);

	if let Ok(ext) = &result
	{
		debug!(
			"Loaded extension: {} ({})",
			ext.get_name(),
			path.to_str().unwrap()
		);
	}

	return result;
}

fn compute_library_name(filename_stem: &str) -> String
{
	let prefix: &str = library_prefix_for_platform();
	return filename_stem[prefix.len()..].to_string();
}

const fn library_extension_for_platform() -> &'static str
{
	return match HOST.operating_system
	{
		OperatingSystem::Windows => "dll",
		OperatingSystem::Linux => "so",
		_ => panic!("Unsupported operating system"),
	};
}

const fn library_prefix_for_platform() -> &'static str
{
	return match HOST.operating_system
	{
		OperatingSystem::Windows => "",
		OperatingSystem::Linux => "lib",
		_ => panic!("Unsupported operating system"),
	};
}
