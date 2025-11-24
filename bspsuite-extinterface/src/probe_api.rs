use crate::probe_api::internal::ExportedApis;

use super::string_ref::StringRef;
use super::{dummy_api, log_api};
use log::{error, trace};
use std::result::Result;

pub const API_VERSION: usize = 1;
pub type ExtFnProbe = extern "C" fn(&mut ProbeApi) -> ProbeResult;

/// Enum representing a failure to provide a requested API to the caller
/// extension.
#[repr(C)]
pub enum RequestError
{
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
		return ExportedApis::request_get_api(
			self.extension_name.to_string().as_str(),
			&mut self.apis.log_api,
			requested_version,
		);
	}

	pub fn register_dummy_api_callbacks(
		&mut self,
		requested_version: usize,
		callbacks: dummy_api::DummyCallbacks,
	) -> Result<(), RequestError>
	{
		return ExportedApis::request_set_callbacks(
			self.extension_name.to_string().as_str(),
			&mut self.apis.dummy_api,
			requested_version,
			callbacks,
		);
	}
}

pub mod internal
{
	use super::*;

	#[repr(C)]
	pub enum ApiRequestError
	{
		// Requested -> Actual
		MismatchedVersion((usize, usize)),
	}

	#[repr(C)]
	pub struct ApiProvider<T>
	where
		T: Clone,
	{
		name: StringRef<'static>,
		version: usize,
		api: T,
	}

	#[repr(C)]
	pub struct CallbacksContainer<T>
	{
		name: StringRef<'static>,
		version: usize,
		callbacks: Option<T>,
	}

	#[repr(C)]
	pub struct ExportedApis
	{
		pub log_api: ApiProvider<log_api::LogApi>,
		pub dummy_api: CallbacksContainer<dummy_api::DummyCallbacks>,
	}

	impl<T> ApiProvider<T>
	where
		T: Clone,
	{
		pub fn new(name: &'static str, version: usize, api: T) -> Self
		{
			return Self {
				name: StringRef::from(name),
				version: version,
				api: api,
			};
		}

		pub fn get_name(&self) -> String
		{
			return self.name.to_string();
		}

		pub fn get_version(&self) -> usize
		{
			return self.version;
		}

		pub fn request_get_api(&self, requested_version: usize) -> Result<T, ApiRequestError>
		{
			if requested_version != self.version
			{
				return Err(ApiRequestError::MismatchedVersion((
					requested_version,
					self.version,
				)));
			}

			return Ok(self.api.clone());
		}
	}

	impl<T> CallbacksContainer<T>
	{
		pub fn new(name: &'static str, version: usize) -> Self
		{
			return Self {
				name: StringRef::from(name),
				version: version,
				callbacks: None,
			};
		}

		pub fn get_name(&self) -> String
		{
			return self.name.to_string();
		}

		pub fn get_version(&self) -> usize
		{
			return self.version;
		}

		pub fn request_set_callbacks(
			&mut self,
			requested_version: usize,
			callbacks: T,
		) -> Result<(), ApiRequestError>
		{
			if requested_version != self.version
			{
				return Err(ApiRequestError::MismatchedVersion((
					requested_version,
					self.version,
				)));
			}

			self.callbacks = Some(callbacks);
			return Ok(());
		}

		pub fn take_callbacks(self) -> Option<T>
		{
			return self.callbacks;
		}
	}

	impl ExportedApis
	{
		pub fn request_get_api<T>(
			extension_name: &str,
			provider: &mut ApiProvider<T>,
			requested_version: usize,
		) -> Result<T, RequestError>
		where
			T: Clone,
		{
			return ExportedApis::process_result(
				extension_name,
				provider.get_name().as_str(),
				provider.get_version(),
				provider.request_get_api(requested_version),
			);
		}

		pub fn request_set_callbacks<T>(
			extension_name: &str,
			container: &mut CallbacksContainer<T>,
			requested_version: usize,
			callbacks: T,
		) -> Result<(), RequestError>
		{
			return ExportedApis::process_result(
				extension_name,
				container.get_name().as_str(),
				container.get_version(),
				container.request_set_callbacks(requested_version, callbacks),
			);
		}

		pub fn process_result<T>(
			extension_name: &str,
			api_name: &str,
			version: usize,
			result: Result<T, ApiRequestError>,
		) -> Result<T, RequestError>
		{
			if let Err(req_err) = &result
			{
				match req_err
				{
					ApiRequestError::MismatchedVersion((requested, actual)) =>
					{
						error!(
							"Extension {extension_name} failed request for {api_name}. Requested version was {requested}, but the provided version is {actual}",
						);
					}
				}
			}
			else
			{
				trace!(
					"Extension {extension_name} successfully requested version {version} of {api_name}",
				);
			}

			return result.map_err(|res| match res
			{
				ApiRequestError::MismatchedVersion((_, actual)) =>
				{
					RequestError::VersionDidNotMatch(actual)
				}
			});
		}
	}
}
