use super::string_ref::StringRef;
use log;

pub const NAME: &str = "LogApi";
pub const API_VERSION: usize = 1;

pub type CanLogFn = extern "C" fn(&log::Level) -> bool;
pub type LogFn = extern "C" fn(&LogMessageArgs);

#[repr(C)]
#[derive(Clone)]
pub struct LogApi
{
	pub can_log: CanLogFn,
	pub log_fn: LogFn,
}

#[repr(C)]
pub struct LogMessageArgs<'l>
{
	pub level: log::Level,
	pub target: &'l str,
	pub module: &'l str,
	pub file: &'l str,
	pub line: u32,
	pub msg: StringRef<'l>,
}

pub struct ExtensionLogger
{
	log_api: LogApi,
}

impl ExtensionLogger
{
	pub fn new(log_api: LogApi) -> Box<Self>
	{
		return Box::new(Self { log_api: log_api });
	}
}

impl log::Log for ExtensionLogger
{
	fn enabled(&self, metadata: &log::Metadata) -> bool
	{
		return (self.log_api.can_log)(&metadata.level());
	}

	fn log(&self, record: &log::Record)
	{
		if self.enabled(record.metadata())
		{
			let message: String = format!("{}", record.args());

			let args: LogMessageArgs = LogMessageArgs {
				level: record.metadata().level(),
				target: record.metadata().target(),
				module: record.module_path().unwrap_or("<unknown>"),
				file: record.file().unwrap_or("<unknown>"),
				line: record.line().unwrap_or(0),
				msg: StringRef::from(message.as_ref()),
			};

			(self.log_api.log_fn)(&args);
		}
	}

	fn flush(&self)
	{
	}
}
