use super::api_info::ApiInfo;

pub const API_INFO: ApiInfo = ApiInfo::new("MapParserApi", 1);

#[repr(C)]
pub struct MapParserApi {}

#[repr(C)]
#[derive(Clone)]
pub struct MapParserCallbacks
{
	pub register_map_formats: extern "C" fn(),
}
