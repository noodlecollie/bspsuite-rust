use std::path::PathBuf;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::error_strings;
use anyhow::{Context, Result, bail};
use simplelog::{debug, warn};

use crate::extensions::{
	BSPSUITE_EXT_INTERFACE_CURRENT_VERSION, Extension, find_extensions, load_extensions,
};

pub struct LibState
{
	pub extensions: Vec<Extension>,

	// Not very idiomatic, but since we want to wrap this struct
	// directly in a RwLock, we want to avoid needing an Option.
	// This is always checked before clients are returned a lock.
	valid: bool,

	// Has to be a string so that we can const-initialise.
	toolchain_root: String,

	verbose: bool,
}

impl Default for LibState
{
	fn default() -> LibState
	{
		return LibState::const_default();
	}
}

impl LibState
{
	const fn const_default() -> LibState
	{
		return LibState {
			valid: false,
			toolchain_root: String::new(),
			extensions: Vec::new(),
			verbose: false,
		};
	}

	pub fn new(toolchain_root: String) -> Result<LibState>
	{
		let extensions_dir: PathBuf = PathBuf::from(&toolchain_root).join("extensions");
		let extension_paths: Vec<PathBuf> = find_extensions(&extensions_dir)?;

		debug!(
			"Found {} extensions in {}",
			extension_paths.len(),
			extensions_dir.to_str().unwrap()
		);

		let extensions: Vec<Result<Extension>> =
			load_extensions(&extension_paths, BSPSUITE_EXT_INTERFACE_CURRENT_VERSION);

		for extension in extensions.iter().filter(|ext| ext.is_err())
		{
			let err = extension.as_ref().err().unwrap();
			warn!("{}", error_strings::log_string(err));
		}

		let extensions: Vec<Extension> =
			extensions.into_iter().filter_map(|ext| ext.ok()).collect();

		return Ok(LibState {
			valid: true,
			toolchain_root: toolchain_root,
			extensions: extensions,
			..Default::default()
		});
	}

	pub fn is_valid(&self) -> bool
	{
		return self.valid;
	}

	pub fn verbose(&self) -> bool
	{
		return self.verbose;
	}
}

static LIBSTATE: RwLock<LibState> = RwLock::new(LibState::const_default());

pub fn initialise(toolchain_root: &Option<PathBuf>) -> Result<()>
{
	let root: PathBuf = compute_toolchain_root(toolchain_root)?;
	let root_str: &str = root
		.to_str()
		.with_context(|| "Could not convert toolchain root path to string")?;

	let lock = LIBSTATE.try_write();

	if let Err(_) = lock
	{
		bail!("Could not acquire write lock for lib state");
	}

	let mut state: RwLockWriteGuard<'_, LibState> = lock.unwrap();

	if state.is_valid()
	{
		bail!("Cannot initialise state that's already been initialised earlier");
	}

	*state = LibState::new(String::from(root_str))?;
	Ok(())
}

pub fn destroy() -> Result<()>
{
	let lock = LIBSTATE.try_write();

	if let Err(_) = lock
	{
		bail!("Could not acquire write lock for lib state");
	}

	let mut state: RwLockWriteGuard<'_, LibState> = lock.unwrap();

	*state = LibState::const_default();
	Ok(())
}

// Expects no write lock, panics if there is one active.
// Panics if the state is not valid.
pub fn get() -> RwLockReadGuard<'static, LibState>
{
	let lock: RwLockReadGuard<'static, LibState> = LIBSTATE
		.try_read()
		.expect("Could not acquire read lock for lib state");

	if !lock.is_valid()
	{
		panic!("Lib state was not valid");
	}

	return lock;
}

fn compute_toolchain_root(root_override: &Option<PathBuf>) -> Result<PathBuf>
{
	return match root_override
	{
		// We were provided a path. Make sure it's a directory.
		Some(root) =>
		{
			if root.is_dir()
			{
				Ok(root.clone())
			}
			else
			{
				bail!("Provided toolchain root was not a directory or could not be accessed")
			}
		}

		// We were not provided a path. Use the current executable's directory.
		None => match std::env::current_exe()
		{
			Ok(exe_path) => match exe_path.parent()
			{
				Some(parent_path) => Ok(parent_path.to_path_buf()),
				None => bail!("Could not get parent directory of current executable.",),
			},
			Err(err) => bail!("Could not get path to current executable. {err}"),
		},
	};
}
