use super::log_api;
use super::string_ref::StringRef;
use log::{error, trace};
use std::result::Result;

pub const API_VERSION: usize = 1;
pub type ExtFnProbe = extern "C" fn(&mut ProbeApi) -> ProbeResult;

/// Enum representing a failure to provide a requested API to the caller
/// extension.
#[repr(C)]
pub enum RequestError
{
	/// The extension already successfully requested the API earlier.
	AlreadyObtained,

	/// The provided version did not match the version of the available API.
	/// The inner value of this enum item is the actual version available.
	VersionDidNotMatch(usize),
}

/// Enum representing the result of a probe call to an extension.
#[repr(C)]
pub enum ProbeResult
{
	/// The extension was able to obtain all the APIs that it needed. This
	/// result still covers cases where an extension is not able to get every
	/// single API that it asks for, but is still able to operate correctly.
	Success,

	/// The extension was not able to obtain all the APIs it needed to function
	/// correctly.
	Failure,
}

#[repr(C)]
pub struct ProbeApi<'l>
{
	extension_name: StringRef<'l>,
	apis: internal::ExportedApis,
}

impl<'l> ProbeApi<'l>
{
	pub fn new(extension_name: &'l str, apis: internal::ExportedApis) -> ProbeApi<'l>
	{
		return ProbeApi {
			extension_name: StringRef::from(extension_name),
			apis: apis,
		};
	}

	pub fn request_log_api(
		&mut self,
		requested_version: usize,
	) -> Result<log_api::LogApi, RequestError>
	{
		return self.apis.log_api.request_version(
			self.extension_name.to_string().as_str(),
			requested_version,
			(),
		);
	}
}

pub mod internal
{
	use super::*;

	#[repr(C)]
	pub enum ApiRequestResult
	{
		None,
		Taken,

		// Version stored here is the version that was requested,
		// so that we can look this up later if we want to know.
		RequestedIncompatibleVersion(usize),
	}

	#[repr(C)]
	pub struct ExportedApi<CoreApi, ExtCallbacks = ()>
	where
		CoreApi: Clone,
		ExtCallbacks: Clone,
	{
		pub name: StringRef<'static>,
		pub version: usize,
		pub core_api: CoreApi,
		pub ext_callbacks: ExtCallbacks,
		pub last_request_result: ApiRequestResult,
	}

	impl<CoreApi, ExtCallbacks> ExportedApi<CoreApi, ExtCallbacks>
	where
		CoreApi: Clone,
		ExtCallbacks: Clone,
	{
		pub fn new(
			name: &'static str,
			version: usize,
			core_api: CoreApi,
			ext_callbacks: ExtCallbacks,
		) -> Self
		{
			return Self {
				name: StringRef::from(name),
				version: version,
				core_api: core_api,
				ext_callbacks: ext_callbacks,
				last_request_result: ApiRequestResult::None,
			};
		}

		pub fn request_version(
			&mut self,
			extension_name: &str,
			requested_version: usize,
			ext_callbacks: ExtCallbacks,
		) -> Result<CoreApi, RequestError>
		{
			let latest_request_result: ApiRequestResult = {
				if let ApiRequestResult::Taken = self.last_request_result
				{
					error!(
						"Extension {extension_name} requested {} when it had already been consumed by an earlier call",
						self.name.to_string()
					);

					ApiRequestResult::Taken
				}
				else if requested_version != self.version
				{
					error!(
						"Extension {extension_name} failed request for {}. Requested version was {requested_version}, but the provided version is {}",
						self.name.to_string(),
						self.version
					);

					ApiRequestResult::RequestedIncompatibleVersion(requested_version)
				}
				else
				{
					trace!(
						"Extension {extension_name} successfully requested version {requested_version} of {}",
						self.name.to_string(),
					);

					self.ext_callbacks = ext_callbacks;
					ApiRequestResult::Taken
				}
			};

			let out_result: Result<CoreApi, RequestError> = match latest_request_result
			{
				ApiRequestResult::Taken =>
				{
					if let ApiRequestResult::None = self.last_request_result
					{
						Ok(self.core_api.clone())
					}
					else
					{
						Err(RequestError::AlreadyObtained)
					}
				}
				ApiRequestResult::RequestedIncompatibleVersion(_) =>
				{
					Err(RequestError::VersionDidNotMatch(self.version))
				}
				_ => unreachable!(),
			};

			self.last_request_result = latest_request_result;
			return out_result;
		}
	}

	#[repr(C)]
	pub struct ExportedApis
	{
		pub log_api: ExportedApi<log_api::LogApi>,
	}
}
