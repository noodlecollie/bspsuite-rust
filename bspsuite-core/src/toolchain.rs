use super::extensions::ExtensionList;
use std::path::PathBuf;

pub struct Toolchain
{
	root: PathBuf,
	extensions: ExtensionList,
}

impl Toolchain
{
	pub fn new(toolchain_root: &Option<PathBuf>) -> Self
	{
		let root_path: PathBuf = if toolchain_root.is_some()
		{
			toolchain_root.as_ref().unwrap().clone()
		}
		else
		{
			Toolchain::infer_toolchain_root()
		};

		return Self {
			root: root_path.clone(),
			extensions: ExtensionList::new(&root_path),
		};
	}

	pub fn extensions(&self) -> &ExtensionList
	{
		return &self.extensions;
	}

	fn infer_toolchain_root() -> PathBuf
	{
		let exe_path: PathBuf =
			std::env::current_exe().expect("Could not get path to current executable");

		return exe_path.parent().unwrap().to_path_buf();
	}
}
