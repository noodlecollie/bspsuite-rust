use super::extensions::ExtensionList;
use anyhow::{Result, bail};
use std::path::PathBuf;
use std::sync::{Arc, RwLock, Weak};

pub struct CompilerState
{
	toolchain_root: PathBuf,
	extensions: ExtensionList,
}

static STATE_WEAKPTR: RwLock<Weak<CompilerState>> = RwLock::new(Weak::new());

impl CompilerState
{
	pub fn new_static(toolchain_root: &Option<PathBuf>) -> Result<Arc<Self>>
	{
		let lock_result = STATE_WEAKPTR.try_write();

		if lock_result.is_err()
		{
			bail!("Could not acquire write lock for compiler state");
		}

		let mut lock = lock_result.unwrap();

		let root_path: PathBuf = if toolchain_root.is_some()
		{
			toolchain_root.as_ref().unwrap().clone()
		}
		else
		{
			CompilerState::infer_toolchain_root()
		};

		let state: Arc<Self> = Arc::new(Self {
			toolchain_root: root_path.clone(),
			extensions: ExtensionList::new(&root_path),
		});

		*lock = Arc::downgrade(&state);

		return Ok(state);
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
