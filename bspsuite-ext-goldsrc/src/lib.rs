use bspextifc::log_api::{self, ExtensionLogger};
use bspextifc::{dummy_api, implement_extension_info, probe_api};
use log::{info, trace};

mod io;

implement_extension_info!(probe);

extern "C" fn probe(api: &mut probe_api::ProbeApi)
{
	let log_api_result: Result<log_api::LogApi, probe_api::RequestError> =
		api.request_log_api(log_api::API_VERSION);

	if let Ok(api) = log_api_result
	{
		// Not really much we can do here if we're unable to log.
		// Perhaps we should return a failure code from the probe function?
		if ExtensionLogger::assign_static_logger(api).is_ok()
		{
			// REMOVE ME once tested
			trace!("Goldsrc extension set logger successfully");
		}
	}
}

extern "C" fn dummyapi_entry_point(api: &mut dummy_api::DummyApi)
{
	let magic_number: i32 = api.get_magic_number();
	info!("Magic number from dummy API: {magic_number}");
}
