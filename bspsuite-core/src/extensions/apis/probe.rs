use super::dummy;
use log::error;

/// Enum to indicate whether an API at a given version is supported.
#[repr(C)]
pub enum ApiSupported
{
	/// The API is supported.
	Yes,

	// The API is not supported. The enum value contains the version number that is supported.
	No(usize),
}

#[repr(C)]
pub struct ProbeApi
{
	extension_name: String,
	dummy_callbacks: dummy::DummyCallbacks,
}

impl ProbeApi
{
	pub extern "C" fn request_dummy_api(
		&mut self,
		version: usize,
		callbacks: dummy::DummyCallbacks,
	) -> ApiSupported
	{
		if let Err(actual_version) = self.check_version("DummyApi", version, dummy::API_VERSION)
		{
			return ApiSupported::No(actual_version);
		}

		self.dummy_callbacks = callbacks;
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

		return Ok(());
	}
}
