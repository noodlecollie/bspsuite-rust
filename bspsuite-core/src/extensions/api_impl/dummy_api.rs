use bspextifc::dummy_api;
use std::ffi::c_void;
use std::marker::PhantomData;

struct DummyApiImpl
{
	magic_number: i32,
	numbers: Vec<i32>,
}

impl DummyApiImpl
{
	pub fn new(magic_number: i32) -> Self
	{
		return Self {
			magic_number: magic_number,
			numbers: Vec::new(),
		};
	}

	pub fn get_magic_number(&self) -> i32
	{
		return self.magic_number;
	}

	pub fn store_number(&mut self, value: i32)
	{
		self.numbers.push(value);
	}
}

pub fn call_dummy_api(entry_point: dummy_api::EntryPointFn)
{
	let mut api_impl: DummyApiImpl = DummyApiImpl::new(42);
	let mut core_fns: dummy_api::internal::DummyApiCoreFns = dummy_api::internal::DummyApiCoreFns {
		context: &mut api_impl as *mut DummyApiImpl as *mut c_void,
		phantom: PhantomData,
		store_number_fn: store_number,
		get_magic_number_fn: get_magic_number,
	};

	let mut api: dummy_api::DummyApi = dummy_api::internal::create_dummy_api(&mut core_fns);
	entry_point(&mut api);
}

unsafe extern "C" fn store_number(context: *mut c_void, value: i32)
{
	unsafe { (*context.cast::<DummyApiImpl>()).store_number(value) };
}

unsafe extern "C" fn get_magic_number(context: *const c_void) -> i32
{
	return unsafe { (*context.cast::<DummyApiImpl>()).get_magic_number() };
}
