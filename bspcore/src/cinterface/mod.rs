#![allow(unsafe_code)]

use std::ffi::c_int;
use std::os::raw::c_char;

mod utils;

use super::entrypoint;

// Function prototype taken from https://paandahl.github.io/rust-interop/c/shared-types.html
pub extern "C" fn libbspsuite_entrypoint(argc: usize, argv: *const *const c_char) -> c_int
{
	return entrypoint(&utils::unsafe_argv_to_string_vec(argc, argv));
}
