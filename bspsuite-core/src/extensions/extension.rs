use super::loader::{UnsafeSymbol, get_unsafe_symbol};
use anyhow::Result;
use bspextifc::{dummy_api, probe_api};
use libloading::Library;

struct ApiCallbacks
{
	dummy_api_callbacks: Option<dummy_api::DummyCallbacks>,
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

	probe_fn: UnsafeSymbol<probe_api::ExtFnProbe>,
	api_callbacks: ApiCallbacks,
}

impl Extension
{
	pub fn from(name: String, library: Library) -> Result<Extension>
	{
		// Unsafe symbol calls are allowed here, since all the symbols are stored
		// privately on the struct, and the struct cannot live longer than the library.
		let probe_fn: UnsafeSymbol<probe_api::ExtFnProbe> =
			unsafe { get_unsafe_symbol(&library, probe_api::SYMBOL_PROBE)? };

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
		let mut probe_callbacks: probe_api::internal::ExtensionCallbacks =
			probe_api::internal::ExtensionCallbacks::default();

		let mut api: probe_api::ProbeApi =
			probe_api::ProbeApi::new(self.name.as_str(), &mut probe_callbacks);

		(*self.probe_fn)(&mut api);

		self.api_callbacks = ApiCallbacks {
			dummy_api_callbacks: probe_callbacks.dummy_callbacks,
		};

		return Ok(());
	}
}
