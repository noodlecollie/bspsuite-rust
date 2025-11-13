mod cli;

use std::ffi::{CStr, CString, c_char};

use bspcore::{self, ResultCode};
use clap::Parser;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode, error, info};

use crate::cli::DebugLevel;

fn main()
{
	let parsed_args: cli::Cli = cli::Cli::parse();

	init_logger(&parsed_args);
	print_banner();

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
	let args: bspcore::CompileArgs = bspcore::CompileArgs {
		base: bspcore::BaseArgs::default(),
		input_file: args.input_file.clone(),
	};

	return bspcore::bspcore_run_compile_command(&args);
}

// TODO: Swap to https://github.com/daboross/fern?
// We'd like to customise the log output more than is
// possible here. Hopefully we can also use Paris
// (https://github.com/0x20f/paris)
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

	let mut builder: ConfigBuilder = ConfigBuilder::new();
	builder.set_time_level(LevelFilter::Off);

	TermLogger::init(
		log_filter,
		builder.build(),
		TerminalMode::Mixed,
		ColorChoice::Auto,
	)
	.expect("Could not initialise logger");
}

fn print_banner()
{
	let build_id_ptr: *const c_char = bspcore::bspcore_get_build_identifier_string();
	let build_id: &'static CStr = unsafe { CStr::from_ptr(build_id_ptr) };

	info!(
		"\n\
		================================================================================\n\
		<b>{}</b> version {} ({})\n\
		================================================================================",
		env!("CARGO_BIN_NAME"),
		env!("CARGO_PKG_VERSION"),
		build_id.to_str().unwrap()
	);
}
