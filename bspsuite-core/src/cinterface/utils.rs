use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;

pub fn unsafe_argv_to_string_vec(argc: usize, argv: *const *const c_char) -> Vec<String>
{
	assert!(argc > 0, "Expected a non-zero number of arguments");
	assert!(argv != std::ptr::null(), "Expected argv to be valid");

	let arg_slice: &[*const i8] = unsafe { slice::from_raw_parts(argv, argc) };

	// Following approach described here: https://stackoverflow.com/a/38186733/2054335
	return arg_slice
		.iter()
		.map(|&item| unsafe { CStr::from_ptr(item) }.to_str().unwrap().into())
		.collect();
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	#[should_panic]
	fn argv_to_stringvec_argc_nonzero_argv_null()
	{
		unsafe_argv_to_string_vec(5, std::ptr::null());
	}

	#[test]
	#[should_panic]
	fn argv_to_stringvec_argc_zero_argv_null()
	{
		unsafe_argv_to_string_vec(0, std::ptr::null());
	}

	#[test]
	#[should_panic]
	fn argv_to_stringvec_argc_zero_argv_valid()
	{
		let argv_vec: Vec<*const c_char> = ["one\0", "two\0", "three\0"]
			.iter()
			.map(|string| string.as_ptr() as *const _)
			.collect();

		unsafe_argv_to_string_vec(0, argv_vec.as_ptr());
	}

	#[test]
	fn argv_to_stringvec_normal_args()
	{
		let argv_vec: Vec<*const c_char> = ["one\0", "two\0", "three\0"]
			.iter()
			.map(|string| string.as_ptr() as *const _)
			.collect();

		let strings: Vec<String> = unsafe_argv_to_string_vec(argv_vec.len(), argv_vec.as_ptr());

		assert_eq!(*strings.get(0).unwrap(), String::from("one"));
		assert_eq!(*strings.get(1).unwrap(), String::from("two"));
		assert_eq!(*strings.get(2).unwrap(), String::from("three"));
	}
}
