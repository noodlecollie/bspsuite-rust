use bspcore::{
	BSPSUITE_EXT_INTERFACE_CURRENT_VERSION, ExtensionServicesApi, ExtensionServicesResult,
};

mod io;

#[unsafe(no_mangle)]
extern "C" fn bspsuite_ext_get_interface_version() -> usize
{
	return BSPSUITE_EXT_INTERFACE_CURRENT_VERSION;
}

#[unsafe(no_mangle)]
extern "C" fn bspsuite_ext_present_services(api: &ExtensionServicesApi) -> ExtensionServicesResult
{
	println!("FROM EXT-GOLDSRC: api.temp() returned {}", api.temp());
	return ExtensionServicesResult::Ok;
}
