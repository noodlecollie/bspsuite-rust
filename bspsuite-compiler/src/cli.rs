use std::path::PathBuf;

/// Enum representing a numeric error code that may be returned from the
/// compiler executable. A value of 0 (not present here) would indicate success.
#[derive(Copy, Clone, Debug, strum::Display)]
pub enum ErrorCode
// TODO: Move to core lib
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

#[derive(Copy, Clone, Debug, strum::Display, clap::ValueEnum)]
pub enum DebugLevel
{
	/// No debug logging will occur.
	Off,

	/// Normal debug logs will be printed.
	On,

	/// Debug and trace logs will be printed.
	Trace,
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli
{
	#[arg(short, long)]
	pub debug: Option<DebugLevel>,

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
