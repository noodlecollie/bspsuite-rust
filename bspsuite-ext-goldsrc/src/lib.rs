use bspcore::extensions::{
	DummyApi, ExtensionServicesApi, ExtensionServicesResult, INTERFACE_VERSION,
};

mod io;

#[unsafe(no_mangle)]
extern "C" fn bspsuite_ext_get_interface_version() -> usize
{
	return INTERFACE_VERSION;
}

#[unsafe(no_mangle)]
extern "C" fn bspsuite_ext_present_services(
	api: &mut ExtensionServicesApi,
) -> ExtensionServicesResult
{
	let dummy_api: DummyApi = api.request_dummy_api(1);

	return if dummy_api.is_valid()
	{
		ExtensionServicesResult::Ok
	}
	else
	{
		ExtensionServicesResult::Missed
	};
}
