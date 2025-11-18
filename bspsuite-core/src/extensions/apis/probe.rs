/// Enum to indicate whether an API at a given version is supported.
#[repr(C)]
pub enum ApiSupported
{
	/// The API is supported.
	Yes,

	// The API is not supported. The enum value contains the version number that is supported.
	No(usize),
}

pub struct ProbeApi {}

impl ProbeApi
{
	pub fn request_dummy_api(&self, version: usize) -> ApiSupported
	{
		// TODO
		return ApiSupported::No(0);
	}
}
