use anyhow::Result;
use std::path::PathBuf;

use super::extension::Extension;
use super::loader::{find_extensions, load_extensions};
use log::{debug, warn};

pub struct ExtensionList
{
	extensions: Vec<Extension>,
}

impl ExtensionList
{
	pub fn new(toolchain_root: &PathBuf) -> Self
	{
		let mut out: Self = Self {
			extensions: Vec::new(),
		};

		out.load_extensions(toolchain_root);
		return out;
	}

	pub fn get_extensions(&self) -> &[Extension]
	{
		return &self.extensions;
	}

	fn load_extensions(&mut self, toolchain_root: &PathBuf)
	{
		let extensions_dir: PathBuf = toolchain_root.join("extensions");
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

		let extensions: Vec<Result<Extension>> = load_extensions(&extension_paths);

		for extension in extensions.iter().filter(|ext| ext.is_err())
		{
			// TODO: Better error logging
			let err = extension.as_ref().err().unwrap();
			warn!("{err}");
		}

		// TODO
		// let mut extensions: Vec<Extension> =
		// 	extensions.into_iter().filter_map(|ext| ext.ok()).collect();

		// extensions.retain_mut(|ext| match ext.present_services()
		// {
		// 	ExtensionServicesResult::Ok => true,
		// 	ExtensionServicesResult::Missed =>
		// 	{
		// 		warn!(
		// 			"Extension {} was unable to fulfil all of its required APIs",
		// 			ext.get_name(),
		// 		);

		// 		false
		// 	}
		// });

		self.extensions = extensions.into_iter().filter_map(|ext| ext.ok()).collect();
	}
}
