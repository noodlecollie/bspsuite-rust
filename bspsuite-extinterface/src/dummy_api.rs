pub const NAME: &str = "DummyApi";
pub const API_VERSION: usize = 1;

#[repr(C)]
pub struct DummyApi<'l>
{
	data: &'l mut internal::Data,
}

#[repr(C)]
#[derive(Clone)]
pub struct DummyCallbacks
{
	pub entry_point: extern "C" fn(&mut DummyApi),
}

impl<'l> DummyApi<'l>
{
	pub fn new(data: &'l mut internal::Data) -> Self
	{
		return Self { data: data };
	}

	pub extern "C" fn get_magic_number(&self) -> i32
	{
		return 1234;
	}

	pub extern "C" fn store_number(&mut self, value: i32)
	{
		self.data.store_number(value);
	}
}

pub mod internal
{
	#[repr(C)]
	pub struct Data
	{
		pub list: Vec<i32>,
	}

	impl Data
	{
		pub fn new() -> Self
		{
			return Self { list: Vec::new() };
		}

		pub fn store_number(&mut self, value: i32)
		{
			self.list.push(value);
		}
	}
}
