use super::api_info::ApiInfo;
use std::ffi::c_void;
use std::marker::PhantomData;

pub const API_INFO: ApiInfo = ApiInfo::new("DummyApi", 1);

pub type EntryPointFn = extern "C" fn(&mut DummyApi);

#[repr(C)]
pub struct DummyApi<'l>
{
	fns: &'l mut internal::DummyApiCoreFns<'l>,
}

#[repr(C)]
#[derive(Clone)]
pub struct DummyCallbacks
{
	pub entry_point: EntryPointFn,
}

impl<'l> DummyApi<'l>
{
	pub extern "C" fn get_magic_number(&self) -> i32
	{
		return self.fns.get_magic_number();
	}

	pub extern "C" fn store_number(&mut self, value: i32)
	{
		self.fns.store_number(value);
	}
}

pub mod internal
{
	use super::*;

	#[repr(C)]
	pub struct DummyApiCoreFns<'l>
	{
		pub context: *mut c_void,
		pub phantom: PhantomData<&'l c_void>,

		// All unsafe functions here expect that the function implementation
		// converts the type-erased context back into the correct object.
		pub store_number_fn: unsafe extern "C" fn(*mut c_void, i32),
		pub get_magic_number_fn: unsafe extern "C" fn(*const c_void) -> i32,
	}

	impl<'l> DummyApiCoreFns<'l>
	{
		pub fn store_number(&mut self, value: i32)
		{
			unsafe { (self.store_number_fn)(self.context, value) };
		}

		pub fn get_magic_number(&self) -> i32
		{
			return unsafe { (self.get_magic_number_fn)(self.context) };
		}
	}

	pub fn create_dummy_api<'l>(fns: &'l mut internal::DummyApiCoreFns<'l>) -> DummyApi<'l>
	{
		return DummyApi { fns: fns };
	}
}
