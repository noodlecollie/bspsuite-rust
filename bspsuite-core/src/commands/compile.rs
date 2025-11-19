use super::types::{BaseArgs, ResultCode};
use super::utils::wrap_panics;
use crate::compiler_state::CompilerState;
use log::info;
use std::path::PathBuf;

#[repr(C)]
pub struct CompileArgs
{
	pub base: BaseArgs,
	pub input_file: PathBuf,
}

#[unsafe(no_mangle)]
pub extern "C" fn bspcore_run_compile(args: &CompileArgs) -> ResultCode
{
	return wrap_panics(|| {
		let compiler_state: CompilerState = CompilerState::new(&args.base.toolchain_root);

		// TODO: Continue here

		info!("Compile complete");
		return ResultCode::Ok;
	});
}
