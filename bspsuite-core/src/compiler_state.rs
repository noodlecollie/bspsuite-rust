use anyhow::Result;
use simplelog::{debug, warn};
use std::path::PathBuf;

use super::extensions::{
	BSPSUITE_EXT_INTERFACE_CURRENT_VERSION, Extension, find_extensions, load_extensions,
};

pub struct CompilerState
{
	toolchain_root: PathBuf,

	// List of loaded extension libraries.
	extensions: Vec<Extension>,
}

impl CompilerState
{
	pub fn new(toolchain_root: &PathBuf) -> Self
	{
		let mut compiler_state: CompilerState = CompilerState {
			toolchain_root: toolchain_root.clone(),
			extensions: Vec::new(),
		};

		compiler_state.load_extensions();
		return compiler_state;
	}

	fn load_extensions(&mut self)
	{
		let extensions_dir: PathBuf = self.toolchain_root.join("extensions");
		let extensions_result: Result<Vec<PathBuf>> = find_extensions(&extensions_dir);

		if let Err(err) = extensions_result
		{
			warn!("Failed to look up extensions on disk. {}", err);
			return;
		}

		let extension_paths: Vec<PathBuf> = extensions_result.unwrap();

		debug!(
			"Found {} extensions in {}",
			extension_paths.len(),
			extensions_dir.to_str().unwrap()
		);

		let extensions: Vec<Result<Extension>> =
			load_extensions(&extension_paths, BSPSUITE_EXT_INTERFACE_CURRENT_VERSION);

		for extension in extensions.iter().filter(|ext| ext.is_err())
		{
			// TODO: Better error logging
			let err = extension.as_ref().err().unwrap();
			warn!("{err}");
		}

		self.extensions = extensions.into_iter().filter_map(|ext| ext.ok()).collect();
	}
}
