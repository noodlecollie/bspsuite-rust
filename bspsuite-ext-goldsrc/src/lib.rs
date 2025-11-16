use bspcore::{
	BSPSUITE_EXT_INTERFACE_CURRENT_VERSION, DummyApi, ExtensionServicesApi, ExtensionServicesResult,
};

mod io;

#[unsafe(no_mangle)]
extern "C" fn bspsuite_ext_get_interface_version() -> usize
{
	return BSPSUITE_EXT_INTERFACE_CURRENT_VERSION;
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
