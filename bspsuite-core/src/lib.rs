use std::path::PathBuf;

mod extensions;
mod libstate;
pub mod model;

#[repr(C)]
pub struct InitArgs
{
	/// Directory under which the games and directories folders may
	/// be found. If this property is left invalid, the directory of
	/// the current executable is used.
	/// This should be fine for most cases, but if the bspcore library
	/// is being used as part of another application, it may not be
	/// adequate. In this case, the application should supply the
	/// relevant path here.
	toolchain_root: Option<PathBuf>,
}

#[repr(C)]
pub struct BspSuiteApplication
{
	pub toolchain_root: PathBuf,
}

// For now, this just returns a bool to indicate success.
// This should be improved in time. Don't think we can
// return a Result from an extern "C" function, so we may
// need to have some logging callback that gets set.
#[unsafe(no_mangle)]
pub extern "C" fn bspcore_init(args: &InitArgs) -> bool
{
	return match libstate::initialise(&args.toolchain_root)
	{
		Ok(()) => true,
		Err(_) => false,
	};
}

// For now, this just returns a bool to indicate success.
// This should be improved in time. Don't think we can
// return a Result from an extern "C" function, so we may
// need to have some logging callback that gets set.
#[unsafe(no_mangle)]
pub extern "C" fn bspcore_deinit() -> bool
{
	return match libstate::destroy()
	{
		Ok(()) => true,
		Err(_) => false,
	};
}

#[unsafe(no_mangle)]
pub extern "C" fn bspcore_run_compile_command()
{
	// TODO
	println!("run_compile_command() called")
}
