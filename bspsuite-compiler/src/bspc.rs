mod cli;

use bspcore::{self, ResultCode};
use clap::Parser;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode, error};

use crate::cli::DebugLevel;

fn main()
{
	let parsed_args: cli::Cli = cli::Cli::parse();

	init_logger(&parsed_args);

	let subcommand: &cli::Subcommand = &parsed_args.command;
	let result_code: bspcore::ResultCode = match subcommand
	{
		cli::Subcommand::Compile(args) => run_compile_command(&args),
	};

	match result_code
	{
		ResultCode::Ok => (),
		_ =>
		{
			error!("[{subcommand}] failed.");
		}
	}

	std::process::exit(result_code as i32);
}

fn run_compile_command(args: &cli::CompileCommandArgs) -> bspcore::ResultCode
{
	let base_args: bspcore::BaseArgs = bspcore::BaseArgs {
		toolchain_root: None,
	};

	// TODO: Compile args

	return bspcore::bspcore_run_compile_command(&base_args);
}

fn init_logger(parsed_args: &cli::Cli)
{
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

	TermLogger::init(
		log_filter,
		Config::default(),
		TerminalMode::Mixed,
		ColorChoice::Auto,
	)
	.expect("Could not initialise logger");
}
