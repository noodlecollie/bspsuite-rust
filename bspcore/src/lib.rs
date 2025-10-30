// By default, deny usage of unsafe code.
// We will be using unsafe code in places, but this
// should not need to occur outside the cinterface module.
// Adding this check should make it much easier to detect
// if FFI-like code has spilled out into places it shouldn't be.
#![deny(unsafe_code)]
// Some other warnings that I have historically found it very
// useful to treat as errors in C++. Perhaps Rust's type
// system would render them less necessary as errors, but we
// can remove any of the statements below that become annoying.
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(unused_variables)]

pub mod cinterface;

pub fn entrypoint(args: &Vec<String>) -> i32
{
	println!("Entry point called with {} arguments", args.len());

	for (index, arg) in args.iter().enumerate()
	{
		println!("Arg {index}: {arg}");
	}

	return 0;
}
