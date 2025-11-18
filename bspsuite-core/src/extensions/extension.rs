use super::apis::probe::ProbeApi;
use super::loader::{UnsafeSymbol, get_unsafe_symbol};
use anyhow::Result;
use libloading::Library;

const SYMBOL_PROBE: &[u8] = b"bspsuite_ext_probe";

type ExtFnProbe = extern "C" fn(&mut ProbeApi);

pub struct Extension
{
	name: String,
	library: Library,
	probe_fn: UnsafeSymbol<ExtFnProbe>,
}

impl Extension
{
	pub fn from(name: String, library: Library) -> Result<Extension>
	{
		// Unsafe symbol calls are allowed here, since all the symbols are stored
		// privately on the struct, and the struct cannot live longer than the library.
		let probe_fn: UnsafeSymbol<ExtFnProbe> =
			unsafe { get_unsafe_symbol(&library, SYMBOL_PROBE)? };

		return Ok(Extension {
			name: name,
			library: library,
			probe_fn: probe_fn,
		});
	}

	pub fn get_name(&self) -> &str
	{
		return &self.name;
	}
}
