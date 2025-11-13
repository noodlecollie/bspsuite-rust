use const_cstr::{ConstCStr, const_cstr};
use constcat::concat;
use std::any::Any;
use std::ffi::c_char;
use std::panic::{UnwindSafe, catch_unwind};
use std::path::PathBuf;

use simplelog::error;

mod compiler_state;
mod extensions;
pub mod model;

pub use extensions::{
	BSPSUITE_EXT_INTERFACE_CURRENT_VERSION, ExtensionServicesApi, ExtensionServicesResult,
};

use compiler_state::CompilerState;

static BUILD_IDENTIFIER: ConstCStr =
	const_cstr!(concat!(env!("BUILD_DATE"), " ", env!("VCS_HASH")));

#[derive(Copy, Clone, Debug, strum::Display)]
#[repr(C)]
pub enum ResultCode
{
	/// Execution completed successfully.
	Ok = 0,

	/// Some unexpected error occurred during execution. This should never
	/// usually happen.
	InternalError = 1,

	/// The arguments provided when invoking the operation were not valid.
	ArgumentError = 2,

	/// There was an error configuring the compiler.
	ConfigError = 3,

	/// There was an error reading from or writing to disk.
	IoError = 4,
}

#[repr(C)]
pub struct BaseArgs
{
	/// Directory under which the games and directories folders may
	/// be found. If this property is left invalid, the directory of
	/// the current executable is used.
	/// This should be fine for most cases, but if the bspcore library
	/// is being used as part of another application, it may not be
	/// adequate. In this case, the application should supply the
	/// relevant path here.
	pub toolchain_root: Option<PathBuf>,
}

impl Default for BaseArgs
{
	fn default() -> Self
	{
		return Self {
			toolchain_root: None,
		};
	}
}

#[repr(C)]
pub struct CompileArgs
{
	pub base: BaseArgs,
	pub input_file: PathBuf,
}

#[unsafe(no_mangle)]
pub extern "C" fn bspcore_get_build_identifier_string() -> *const c_char
{
	return BUILD_IDENTIFIER.as_ptr();
}

#[unsafe(no_mangle)]
pub extern "C" fn bspcore_run_compile_command(args: &CompileArgs) -> ResultCode
{
	return wrap_panics(|| {
		let compiler_state: CompilerState = CompilerState::new(&args.base.toolchain_root);

		return ResultCode::Ok;
	});
}

// Ensures that if a panic occurs, we log a fatal error and exit.
fn wrap_panics<F>(func: F) -> ResultCode
where
	F: FnOnce() -> ResultCode + UnwindSafe,
{
	type UnwindError = Box<dyn Any + Send + 'static>;
	let result: Result<ResultCode, UnwindError> = catch_unwind(func);

	return match result
	{
		Ok(result_code) => result_code,
		Err(err) =>
		{
			error!("<u><b><red>Fatal Error</red></b></u>\n{:?}", err);
			ResultCode::InternalError
		}
	};
}
