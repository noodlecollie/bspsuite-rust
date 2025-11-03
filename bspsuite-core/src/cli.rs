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
	pub fn exit_code(error_code: &ErrorCode) -> i32
	{
		return *error_code as i32;
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

#[derive(clap::Subcommand)]
pub enum Subcommand
{
	/// Compile a map from a source file.
	Compile
	{
		/// Path to output file. If this exists, it will be overwritten.
		#[arg(short, long)]
		output_file: PathBuf,
	},
}
