use bspcore::extensions::{DummyApi, DummyCallbacks, INTERFACE_VERSION, ProbeApi};

mod io;

#[unsafe(no_mangle)]
extern "C" fn bspsuite_ext_get_interface_version() -> usize
{
	return INTERFACE_VERSION;
}

#[unsafe(no_mangle)]
extern "C" fn bspsuite_ext_probe(api: &mut ProbeApi)
{
	api.request_dummy_api(
		1,
		DummyCallbacks {
			entry_point: dummyapi_entry_point,
		},
	);
}

extern "C" fn dummyapi_entry_point(api: &mut DummyApi)
{
	api.get_magic_number();
}
