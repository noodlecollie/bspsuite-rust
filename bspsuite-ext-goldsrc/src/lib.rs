// By default, deny usage of unsafe code.
// We will be using unsafe code in places, but this
// should not need to occur outside the cinterface module.
// Adding this check should make it much easier to detect
// if FFI-like code has spilled out into places it shouldn't be.
#![deny(unsafe_code)]

pub mod io;
