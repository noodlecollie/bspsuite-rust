use super::ExtensionId;
use super::helpers::UnsupportedVersionError;
use anyhow::Result;

#[repr(C)]
pub struct DummyApi
{
	extension_id: ExtensionId,
}

const CURRENT_VERSION: usize = 1;

pub fn request_dummy_api(
	extension_id: usize,
	version: usize,
) -> Result<DummyApi, UnsupportedVersionError>
{
	if version != CURRENT_VERSION
	{
		return Err(UnsupportedVersionError::new(version, CURRENT_VERSION));
	}

	return Ok(DummyApi {
		extension_id: ExtensionId::Some(extension_id),
	});
}

impl DummyApi
{
	pub extern "C" fn is_valid(&self) -> bool
	{
		return self.extension_id.is_valid();
	}

	pub extern "C" fn get_magic_number(&self) -> i32
	{
		return get_magic_number(&self.extension_id).unwrap_or(0);
	}
}

impl Default for DummyApi
{
	fn default() -> Self
	{
		return Self {
			extension_id: ExtensionId::None,
		};
	}
}

fn get_magic_number(extension_id: &ExtensionId) -> Result<i32>
{
	// TODO
	return Ok(0);
}
