use anyhow::{Result, bail};
use bspextifc::{ExtensionInfo, SYMBOL_EXTENSION_INFO, dummy_api, probe_api};
use libloading::{Library, Symbol};
use log::{debug, trace};
use std::path::PathBuf;
use target_lexicon::{HOST, OperatingSystem};

#[cfg(target_os = "linux")]
pub use libloading::os::unix::Symbol as UnsafeSymbol;
#[cfg(target_os = "windows")]
pub use libloading::os::windows::Symbol as UnsafeSymbol;

struct ApiCallbacks
{
	dummy_api_callbacks: Option<dummy_api::DummyCallbacks>,
}

impl Default for ApiCallbacks
{
	fn default() -> Self
	{
		return Self {
			dummy_api_callbacks: None,
		};
	}
}

pub struct Extension
{
	name: String,

	// This is just here to control the lifetime of the library.
	// By the time this object is constructed, we likely won't
	// need to query the library for anything else, so the member
	// might not be used.
	#[expect(dead_code)]
	library: Library,

	// Unsafe symbols are OK here PROVIDED that they are not
	// copied out of this struct. Here, the extension info
	// will not live longer than the library member above.
	extension_info: UnsafeSymbol<bspextifc::ExtensionInfo>,

	api_callbacks: ApiCallbacks,
}

impl Extension
{
	pub fn from(path: &PathBuf) -> Result<Self>
	{
		let library: Library = unsafe { Library::new(path.as_os_str()) }?;

		let extension_info_symbol: UnsafeSymbol<ExtensionInfo> =
			unsafe { Extension::get_unsafe_symbol(&library, SYMBOL_EXTENSION_INFO) }?;

		let extension_info: &ExtensionInfo = &extension_info_symbol;
		let probe_api_version: usize = extension_info.probe_api_version;

		trace!(
			"Extension {} reported probe API version {probe_api_version}",
			path.to_str().unwrap()
		);

		if probe_api_version != probe_api::API_VERSION
		{
			bail!(
				"Required interface version {}, \
				but extension provided interface version {probe_api_version}.",
				probe_api::API_VERSION
			);
		}

		let name: String =
			Extension::compute_library_name(path.file_stem().unwrap().to_str().unwrap());

		let extension: Self = Self {
			name: name,
			library: library,
			extension_info: extension_info_symbol,
			api_callbacks: ApiCallbacks::default(),
		};

		debug!(
			"Loaded extension: {} ({})",
			extension.get_name(),
			path.to_str().unwrap()
		);

		return Ok(extension);
	}

	pub fn get_name(&self) -> &str
	{
		return &self.name;
	}

	pub fn probe(&mut self) -> Result<()>
	{
		let mut probe_callbacks: probe_api::internal::ExtensionCallbacks =
			probe_api::internal::ExtensionCallbacks::default();

		let mut api: probe_api::ProbeApi =
			probe_api::ProbeApi::new(self.name.as_str(), &mut probe_callbacks);

		let probe_fn: probe_api::ExtFnProbe = self.extension_info.probe_fn;
		probe_fn(&mut api);

		self.api_callbacks = ApiCallbacks {
			dummy_api_callbacks: probe_callbacks.dummy_callbacks,
		};

		return Ok(());
	}

	fn compute_library_name(filename_stem: &str) -> String
	{
		let prefix: &str = Extension::library_prefix_for_platform();
		return filename_stem[prefix.len()..].to_string();
	}

	// It is the caller's responsibility that the symbol is not used after the
	// library is unloaded.
	unsafe fn get_unsafe_symbol<T>(library: &Library, name: &[u8]) -> Result<UnsafeSymbol<T>>
	{
		let symbol: Symbol<T> = unsafe { library.get(name) }?;
		return unsafe { Ok(symbol.into_raw()) };
	}

	pub const fn library_extension_for_platform() -> &'static str
	{
		return match HOST.operating_system
		{
			OperatingSystem::Windows => "dll",
			OperatingSystem::Linux => "so",
			_ => panic!("Unsupported operating system"),
		};
	}

	pub const fn library_prefix_for_platform() -> &'static str
	{
		return match HOST.operating_system
		{
			OperatingSystem::Windows => "",
			OperatingSystem::Linux => "lib",
			_ => panic!("Unsupported operating system"),
		};
	}
}
