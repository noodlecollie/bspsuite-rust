use super::dummy;
use log::{error, trace};

/// Symbol name for the bspsuite_ext_probe function.
pub const SYMBOL_PROBE: &[u8] = b"bspsuite_ext_probe";

/// Function signature for the bspsuite_ext_probe function.
/// Extensions should implement this function to register themselves for
/// specific features.
pub type ExtFnProbe = extern "C" fn(&mut ProbeApi);

/// Enum to indicate whether an API at a given version is supported.
#[repr(C)]
pub enum ApiSupported
{
	/// The API is supported.
	Yes,

	/// The API is not supported. The enclosed value contains the version number
	/// that is supported.
	No(usize),
}

#[repr(C)]
pub struct ProbeApi<'l>
{
	extension_name: &'l str,
	callbacks: &'l mut internal::ExtensionCallbacks,
}

impl<'l> ProbeApi<'l>
{
	pub fn new(
		extension_name: &'l str,
		callbacks: &'l mut internal::ExtensionCallbacks,
	) -> ProbeApi<'l>
	{
		return ProbeApi {
			extension_name: extension_name,
			callbacks: callbacks,
		};
	}

	pub fn request_dummy_api(
		&mut self,
		version: usize,
		callbacks: dummy::DummyCallbacks,
	) -> ApiSupported
	{
		if let Err(actual_version) = self.check_version("DummyApi", version, dummy::API_VERSION)
		{
			return ApiSupported::No(actual_version);
		}

		self.callbacks.dummy_callbacks = Some(callbacks);
		return ApiSupported::Yes;
	}

	fn check_version(
		&self,
		api: &str,
		requested_version: usize,
		actual_version: usize,
	) -> Result<(), usize>
	{
		if requested_version != actual_version
		{
			error!(
				"Extension {} failed request for {api}. Requested version was {requested_version}, but the provided version is {actual_version}",
				self.extension_name
			);

			return Err(actual_version);
		}

		trace!(
			"Extension {} successfully requested version {requested_version} of {api}",
			self.extension_name
		);

		return Ok(());
	}
}

mod internal
{
	#[repr(C)]
	pub struct ExtensionCallbacks
	{
		pub dummy_callbacks: Option<super::dummy::DummyCallbacks>,
	}

	impl Default for ExtensionCallbacks
	{
		fn default() -> Self
		{
			return Self {
				dummy_callbacks: None,
			};
		}
	}
}
