use std::path::PathBuf;

/// Enum representing a numeric error code that may be returned from the
/// compiler executable. A value of 0 (not present here) would indicate success.
#[derive(Copy, Clone)]
pub enum ErrorCode
{
	/// Some unexpected error occurred during execution. This should never
	/// usually happen.
	InternalError = 1,

	/// The provided command line options were not valid.
	CommandLineError = 2,

	/// There was an error configuring the compiler.
	ConfigError = 3,

	/// There was an error reading from or writing to disk.
	IoError = 4,
}

impl ErrorCode
{
	pub fn exit_code(&self) -> i32
	{
		return *self as i32;
	}
}

/// Commands that may be executed on the compiler executable.
#[derive(clap::Parser)]
#[command(version, about = "BSPSuite compiler executable", long_about = None)]
pub struct Cli
{
	#[command(subcommand)]
	pub command: Subcommand,
}

#[derive(clap::Subcommand, strum::Display)]
pub enum Subcommand
{
	/// Compile a map from a source file.
	Compile(CompileCommandArgs),
}

#[derive(clap::Args)]
pub struct CompileCommandArgs
{
	/// Path to output file. If this exists, it will be overwritten.
	#[arg(short, long)]
	pub output_file: PathBuf,
}

pub struct CommandError
{
	pub code: ErrorCode,
	pub description: String,
}

impl CommandError
{
	pub fn new(code: ErrorCode, description: &str) -> Self
	{
		return Self {
			code: code,
			description: String::from(description),
		};
	}

	pub fn exit_code(&self) -> i32
	{
		return self.code.exit_code();
	}
}
