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
			let banner: String = colorize_string("<u><b><red>***** CRITICAL FAILURE *****</>");

			// TODO: Does the project configuration support setting a repo?
			// Can we use an environment variable to fetch the URL?
			error!(
				"\n
				{banner}\n
				The compiler has encountered an unrecoverable error and halted.\n
				Please create an issue report at https://github.com/noodlecollie/bspsuite/issues/\n
				and include the following information:\n
				\n
				{:?}",
				err
			);

			ResultCode::InternalError
		}
	};
}
