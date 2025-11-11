use std::error::Error;
use std::fmt;

mod cli;

use bspcore;
use clap::Parser;
use simplelog::{ColorChoice, Config, Level, LevelFilter, TermLogger, TerminalMode, error};

use crate::cli::DebugLevel;

#[derive(Debug)]
struct CommandError
{
	error: cli::ErrorCode,
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
	let parsed_args: cli::Cli = cli::Cli::parse();

	let log_filter: LevelFilter = match parsed_args.debug
	{
		Some(DebugLevel::Off) => LevelFilter::Info,
		Some(DebugLevel::On) => LevelFilter::Debug,
		Some(DebugLevel::Trace) => LevelFilter::Trace,
		None =>
		{
			if cfg!(debug_assertions)
			{
				LevelFilter::Debug
			}
			else
			{
				LevelFilter::Info
			}
		}
	};

	let verbose: bool = match parsed_args.debug
	{
		Some(DebugLevel::On) => true,
		Some(DebugLevel::Trace) => true,
		_ => false,
	};

	TermLogger::init(
		log_filter,
		Config::default(),
		TerminalMode::Mixed,
		ColorChoice::Auto,
	)
	.expect("Could not initialise logger");

	let bspargs: bspcore::InitArgs = bspcore::InitArgs {
		toolchain_root: None,
		verbose: verbose,
	};

	if !bspcore::bspcore_init(&bspargs)
	{
		std::process::exit(cli::ErrorCode::InternalError as i32);
	}

	let subcommand: &cli::Subcommand = &parsed_args.command;
	let result: Result<(), CommandError> = match subcommand
	{
		cli::Subcommand::Compile(args) => run_compile_command(&args),
	};

	bspcore::bspcore_deinit();

	if let Err(e) = result
	{
		error!("[{subcommand}] failed. {e}");
		std::process::exit(e.error as i32);
	}
}

fn run_compile_command(args: &cli::CompileCommandArgs) -> Result<(), CommandError>
{
	bspcore::bspcore_run_compile_command();

	return Ok(());
}
