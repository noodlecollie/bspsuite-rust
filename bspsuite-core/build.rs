use std::{
	env,
	path::{Path, PathBuf},
};

use cbindgen::Language;

fn main()
{
	println!("cargo:rerun-if-changed=src/cinterface/mod.rs");

	let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

	let mut out_dir: PathBuf = get_main_output_dir();
	out_dir.extend(["cbindgen", "bspsuite", "core", "core.h"].iter());

	cbindgen::Builder::new()
		.with_crate(manifest_dir)
		.with_language(Language::C)
		.generate()
		.expect("Unable to generate C bindings")
		.write_to_file(out_dir.to_str().unwrap());
}

fn get_main_output_dir() -> PathBuf
{
	Path::new(env::var("OUT_DIR").unwrap().as_str())
		.ancestors()
		.nth(3)
		.unwrap()
		.to_path_buf()
}
