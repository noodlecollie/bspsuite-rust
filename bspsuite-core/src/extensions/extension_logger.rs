use log::{Level, Metadata, Record, debug, error, info, trace, warn};
use std::ffi::{CStr, CString, c_char};
use std::str::FromStr;

const LOG_INTERFACE_VERSION: usize = 1;

pub struct ExtensionLogger
{
	enabled_fn: extern "C" fn(&Level) -> bool,
	log_fn: extern "C" fn(&Level, *const c_char),
}

pub enum LoggerFailure
{
	IncompatibleInterfaceVersion,
}

impl ExtensionLogger
{
	pub fn new() -> Result<Box<Self>, LoggerFailure>
	{
		if bspcore_log_interface_version() != LOG_INTERFACE_VERSION
		{
			return Err(LoggerFailure::IncompatibleInterfaceVersion);
		}

		return Ok(Box::new(Self {
			enabled_fn: bspcore_log_msg_enabled,
			log_fn: bspcore_log_msg,
		}));
	}
}

impl log::Log for ExtensionLogger
{
	fn enabled(&self, metadata: &Metadata) -> bool
	{
		return (self.enabled_fn)(&metadata.level());
	}

	fn log(&self, record: &Record)
	{
		if self.enabled(record.metadata())
		{
			let msg: String = format!("[{}] {}", record.target(), record.args());
			if let Ok(c_msg) = CString::from_str(&msg)
			{
				(self.log_fn)(&record.level(), c_msg.as_ptr());
			}
		}
	}

	fn flush(&self)
	{
	}
}

#[unsafe(no_mangle)]
extern "C" fn bspcore_log_interface_version() -> usize
{
	return LOG_INTERFACE_VERSION;
}

#[unsafe(no_mangle)]
extern "C" fn bspcore_log_msg_enabled(level: &Level) -> bool
{
	return *level <= log::max_level();
}

// This function assumes that the string is null-terminated.
// This should always be the case if it is called from the
// ExtensionLogger.
#[unsafe(no_mangle)]
extern "C" fn bspcore_log_msg(level: &Level, msg: *const c_char)
{
	let c_str: &CStr = unsafe { CStr::from_ptr(msg) };

	if let Ok(msg) = c_str.to_str()
	{
		match level
		{
			Level::Error => error!("{msg}"),
			Level::Warn => warn!("{msg}"),
			Level::Info => info!("{msg}"),
			Level::Debug => debug!("{msg}"),
			Level::Trace => trace!("{msg}"),
		}
	}
}
