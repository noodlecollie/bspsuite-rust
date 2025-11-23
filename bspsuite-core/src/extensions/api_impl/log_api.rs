use std::fmt::Arguments;

use bspextifc::log_api::{LogApi, LogMessageArgs};
use log::{Record, RecordBuilder};

pub fn create_api() -> LogApi
{
	return LogApi {
		can_log: can_log,
		log_fn: log_message,
	};
}

extern "C" fn can_log(level: &log::Level) -> bool
{
	return *level <= log::max_level();
}

extern "C" fn log_message(args: &LogMessageArgs)
{
	let msg_string: String = args.msg.to_string();
	let msg_args: Arguments = format_args!("{}", msg_string);

	let mut builder: RecordBuilder = RecordBuilder::new();

	let record: Record = builder
		.file(Some(args.file))
		.line(Some(args.line))
		.target(args.target)
		.module_path(Some(args.module))
		.level(args.level)
		.args(msg_args)
		.build();

	log::logger().log(&record);
}
