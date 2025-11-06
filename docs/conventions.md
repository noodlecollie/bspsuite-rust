# Conventions

## Extension Libraries

When creating extension libraries, the following conventions should be followed for creating nested Rust modules:

* `io`: Should contain code concerned with reading or writing files in certain formats. For example: map source file parsers, image loaders, model loaders.
* `model`: Should contain code representing data structures used internally by the compiler. For example: meshes, map structures.
	* Routines in `io` should usually convert between a file on disk and some appropriate data structure from `model`.
* `cinterface`: Should contain external C-compatible interfaces usable by other native libraries or programs. For example: exposing functions that may be called as callbacks by the main compiler.
	* The `cinterface` module should not contain business logic. It should act ass a thin wrapper that converts between C and Rust data types, and marshals data across the library boundary.
