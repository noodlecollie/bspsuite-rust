use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UnsupportedVersionError
{
	requested_version: usize,
	suppported_version: usize,
}

impl UnsupportedVersionError
{
	pub fn new(requested_version: usize, supported_version: usize) -> Self
	{
		return UnsupportedVersionError {
			requested_version: requested_version,
			suppported_version: supported_version,
		};
	}
}

impl fmt::Display for UnsupportedVersionError
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(
			f,
			"Requested version {} did not match supported version {}",
			self.requested_version, self.suppported_version
		)
	}
}

impl Error for UnsupportedVersionError
{
}
