use crate::commands::ResultCode;
use std::error::Error;
use std::fmt;
use {anyhow, strum};

#[derive(Debug, Copy, Clone, strum::Display)]
pub enum CompilerErrorCode
{
	InternalError,
	ArgumentError,
	ConfigError,
	IoError,
}

impl CompilerErrorCode
{
	pub fn get_result_code(&self) -> ResultCode
	{
		return match self
		{
			CompilerErrorCode::InternalError => ResultCode::InternalError,
			CompilerErrorCode::ArgumentError => ResultCode::ArgumentError,
			CompilerErrorCode::ConfigError => ResultCode::ConfigError,
			CompilerErrorCode::IoError => ResultCode::IoError,
		};
	}
}

#[derive(Debug, Clone)]
pub struct CompilerError
{
	pub code: CompilerErrorCode,
	pub description: String,
}

impl CompilerError
{
	pub fn new(code: CompilerErrorCode, description: String) -> Self
	{
		return Self {
			code: code,
			description: description,
		};
	}

	pub fn first_code_in_chain(err: &anyhow::Error) -> Option<CompilerErrorCode>
	{
		let mut out_code: Option<CompilerErrorCode> = None;

		for item in err.chain()
		{
			if let Some(compiler_error) = item.downcast_ref::<CompilerError>()
			{
				out_code = Some(compiler_error.code);
				break;
			}
		}

		return out_code;
	}
}

impl fmt::Display for CompilerError
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}: {}", self.code, self.description)
	}
}

impl Error for CompilerError
{
}
