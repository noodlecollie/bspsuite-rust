use std::fs;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use libloading::{Library, Symbol, library_filename};
use target_lexicon::{HOST, OperatingSystem};

const BSPSUITE_EXT_INTERFACE_CURRENT_VERSION: usize = 1;
const BSPSUITE_EXT_SYM_GETINTERFACEVERSION: &[u8] = b"bspsuite_ext_get_interface_version";
type ExtFnGetInterfaceVersion = unsafe extern "C" fn() -> usize;

const BSPSUITE_EXT_SYM_PRESENT_SERVICES: &[u8] = b"bspsuite_ext_present_services";
type ExtFnPresentServices = unsafe extern "C" fn(&ExtensionServicesApi) -> ExtensionServicesResult;

pub struct Extension
{
	library: Library,
}

impl Extension
{
	pub fn new(library: Library) -> Extension
	{
		return Extension { library: library };
	}
}

#[repr(C)]
pub enum ExtensionServicesResult
{
	Ok,
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
	// let present_services: Symbol<'lib, ExtFnPresentServices> =
	// 	get_interface_to_extension(&library)?;

	return Ok(Extension::new(library));
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

fn get_interface_to_extension<'lib>(
	library: &'lib Library,
) -> Result<Symbol<'lib, ExtFnPresentServices>>
{
	let get_interface_version: Symbol<'lib, ExtFnGetInterfaceVersion> =
		unsafe { library.get(BSPSUITE_EXT_SYM_GETINTERFACEVERSION) }?;

	let interface_version: usize = unsafe { get_interface_version() };

	if interface_version != BSPSUITE_EXT_INTERFACE_CURRENT_VERSION
	{
		bail!(
			"Required interface version {BSPSUITE_EXT_INTERFACE_CURRENT_VERSION}, \
			but extension provided interface version {interface_version}."
		);
	}

	let present_services: Symbol<'lib, ExtFnPresentServices> =
		unsafe { library.get(BSPSUITE_EXT_SYM_PRESENT_SERVICES) }?;

	return Ok(present_services);
}
