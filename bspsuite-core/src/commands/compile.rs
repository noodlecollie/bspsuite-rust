use super::types::{BaseArgs, ResultCode};
use super::utils::wrap_panics;
use crate::toolchain::Toolchain;
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
		let toolchain: Toolchain = Toolchain::new(&args.base.toolchain_root);

		// TODO

		info!("Compile complete");
		return ResultCode::Ok;
	});
}
