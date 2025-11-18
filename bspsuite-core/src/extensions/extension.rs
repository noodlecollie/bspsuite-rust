use crate::extensions::apis;

use super::loader::{UnsafeSymbol, get_unsafe_symbol};
use anyhow::Result;
use apis::dummy::DummyCallbacks;
use apis::probe::{ProbeApi, ProvidedCallbacks};
use libloading::Library;

const SYMBOL_PROBE: &[u8] = b"bspsuite_ext_probe";
type ExtFnProbe = extern "C" fn(&mut ProbeApi);

struct ApiCallbacks
{
	dummy_api_callbacks: Option<DummyCallbacks>,
}

impl Default for ApiCallbacks
{
	fn default() -> Self
	{
		return Self {
			dummy_api_callbacks: None,
		};
	}
}

pub struct Extension
{
	name: String,

	// This is just here to control the lifetime of the library.
	// By the time this object is constructed, we likely won't
	// need to query the library for anything else, so the member
	// might not be used.
	#[expect(dead_code)]
	library: Library,

	probe_fn: UnsafeSymbol<ExtFnProbe>,
	api_callbacks: ApiCallbacks,
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
			api_callbacks: ApiCallbacks::default(),
		});
	}

	pub fn get_name(&self) -> &str
	{
		return &self.name;
	}

	pub fn probe(&mut self) -> Result<()>
	{
		let mut api: ProbeApi = apis::probe::new(self.name.clone());
		(*self.probe_fn)(&mut api);

		let callbacks: ProvidedCallbacks = apis::probe::finish(api);

		self.api_callbacks = ApiCallbacks {
			dummy_api_callbacks: callbacks.dummy_callbacks,
		};

		return Ok(());
	}
}
