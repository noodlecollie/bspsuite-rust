pub const API_VERSION: usize = 1;

#[repr(C)]
pub struct DummyApi {}

#[repr(C)]
pub struct DummyCallbacks
{
	pub entry_point: extern "C" fn(&mut DummyApi),
}

impl DummyApi
{
	pub extern "C" fn get_magic_number(&self) -> i32
	{
		return 1234;
	}
}
