use super::extensions::ExtensionList;
use std::path::PathBuf;

pub struct CompilerState
{
	toolchain_root: PathBuf,
	extensions: ExtensionList,
}

impl CompilerState
{
	pub fn new(toolchain_root: &Option<PathBuf>) -> Self
	{
		let root_path: PathBuf = if toolchain_root.is_some()
		{
			toolchain_root.as_ref().unwrap().clone()
		}
		else
		{
			CompilerState::infer_toolchain_root()
		};

		return CompilerState {
			toolchain_root: root_path.clone(),
			extensions: ExtensionList::new(&root_path),
		};
	}

	pub fn get_dummy_value() -> i32
	{
		return 1234;
	}

	fn infer_toolchain_root() -> PathBuf
	{
		let exe_path: PathBuf =
			std::env::current_exe().expect("Could not get path to current executable");

		return exe_path.parent().unwrap().to_path_buf();
	}
}
