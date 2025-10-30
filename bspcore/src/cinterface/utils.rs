use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;

pub fn unsafe_argv_to_string_vec(argc: usize, argv: *const *const c_char) -> Vec<String>
{
	// Apparently extern "C" functions cannot unwind, so hitting either
	// of these where we'd expect to use them will just abort.
	// Make sure to test these preconditions when not in an extern "C" function.
	assert!(argc > 0, "Expected a non-zero number of arguments");
	assert!(argv != std::ptr::null(), "Expected argv to be valid");

	let arg_slice: &[*const i8] = unsafe { slice::from_raw_parts(argv, argc) };

	// Following approach described here: https://stackoverflow.com/a/38186733/2054335
	return arg_slice
		.iter()
		.map(|&item| unsafe { CStr::from_ptr(item) }.to_str().unwrap().into())
		.collect();
}
