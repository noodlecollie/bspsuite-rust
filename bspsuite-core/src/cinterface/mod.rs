#![allow(unsafe_code)]

use std::ffi::c_int;
use std::os::raw::c_char;

mod utils;

use super::run_from_shell_arguments;

/// Launch the compiler with the provided shell arguments.
#[unsafe(no_mangle)]
pub extern "C" fn bspcore_run_from_args(argc: c_int, argv: *const *const c_char) -> c_int
{
	// Function prototype taken from https://paandahl.github.io/rust-interop/c/shared-types.html
	return run_from_shell_arguments(&utils::unsafe_argv_to_string_vec(argc, argv));
}
