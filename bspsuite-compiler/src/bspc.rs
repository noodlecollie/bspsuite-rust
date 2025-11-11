use std::error::Error;
use std::fmt;

mod cli;

use bspcore;
use clap::Parser;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

/// Enum representing a numeric error code that may be returned from the
/// compiler executable. A value of 0 (not present here) would indicate success.
#[derive(Copy, Clone, Debug, strum::Display)]
enum ErrorCode
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

#[derive(Debug)]
struct CommandError
{
	error: ErrorCode,
	description: String,
}

impl fmt::Display for CommandError
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}: {}", self.error, self.description)
	}
}

impl Error for CommandError
{
}

fn main()
{
	TermLogger::init(
		LevelFilter::Info,
		Config::default(),
		TerminalMode::Mixed,
		ColorChoice::Auto,
	)
	.expect("Could not initialise logger");

	let parsed_args: cli::Cli = cli::Cli::parse();

	let subcommand: &cli::Subcommand = &parsed_args.command;
	let result: Result<(), CommandError> = match subcommand
	{
		cli::Subcommand::Compile(args) => run_compile_command(&args),
	};

	if let Err(e) = result
	{
		eprintln!("[{subcommand}] failed. {e}");
		std::process::exit(e.error as i32);
	}
}

fn run_compile_command(args: &cli::CompileCommandArgs) -> Result<(), CommandError>
{
	bspcore::bspcore_run_compile_command();

	return Ok(());
}
