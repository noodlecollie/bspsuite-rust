use super::types::ResultCode;
use log::error;
use paris::formatter::colorize_string;
use std::any::Any;
use std::panic::{UnwindSafe, catch_unwind};

// Ensures that if a panic occurs, we log a fatal error and exit.
pub fn wrap_panics<F>(func: F) -> ResultCode
where
	F: FnOnce() -> ResultCode + UnwindSafe,
{
	type UnwindError = Box<dyn Any + Send + 'static>;
	let result: Result<ResultCode, UnwindError> = catch_unwind(func);

	return match result
	{
		Ok(result_code) => result_code,
		Err(err) =>
		{
			let prefix: String = colorize_string("<u><b><red>Fatal Error</red></b></u>");
			error!("{prefix}\n{:?}", err);
			ResultCode::InternalError
		}
	};
}
