use bspextifc::{EXTENSION_INTERFACE_VERSION, dummy_api, probe_api};
use log::info;

mod io;

#[unsafe(no_mangle)]
extern "C" fn bspsuite_ext_get_interface_version() -> usize
{
	return EXTENSION_INTERFACE_VERSION;
}

#[unsafe(no_mangle)]
extern "C" fn bspsuite_ext_probe(api: &mut probe_api::ProbeApi)
{
	api.request_dummy_api(
		dummy_api::API_VERSION,
		dummy_api::DummyCallbacks {
			entry_point: dummyapi_entry_point,
		},
	);
}

extern "C" fn dummyapi_entry_point(api: &mut dummy_api::DummyApi)
{
	let magic_number: i32 = api.get_magic_number();
	info!("Magic number from dummy API: {magic_number}");
}
