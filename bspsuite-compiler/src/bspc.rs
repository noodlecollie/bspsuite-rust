mod cli;

use std::ffi::{CStr, c_char};

use bspcore::commands as Cmds;
use clap::Parser;
use lazy_static::lazy_static;
use log::{Level, LevelFilter, error, info};
use paris::formatter::colorize_string;

use crate::cli::DebugLevel;

fn main()
{
	let parsed_args: cli::Cli = cli::Cli::parse();

	init_logger(&parsed_args);
	print_banner();

	let subcommand: &cli::Subcommand = &parsed_args.command;
	let result_code: Cmds::ResultCode = match subcommand
	{
		cli::Subcommand::Compile(args) => run_compile_command(&args),
	};

	match result_code
	{
		Cmds::ResultCode::Ok => (),
		_ =>
		{
			error!("[{subcommand}] failed.");
		}
	}

	std::process::exit(result_code as i32);
}

fn run_compile_command(args: &cli::CompileCommandArgs) -> Cmds::ResultCode
{
	let args: Cmds::CompileArgs = Cmds::CompileArgs {
		base: Cmds::BaseArgs::default(),
		input_file: args.input_file.clone(),
	};

	return Cmds::bspcore_run_compile_command(&args);
}

fn init_logger(parsed_args: &cli::Cli)
{
	lazy_static! {
		pub static ref WARNING_PREFIX: String = colorize_string("<b><yellow>Warning:</>");
		pub static ref ERROR_PREFIX: String = colorize_string("<b><red>Error:</>");
	}

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

	let base_config = fern::Dispatch::new().level(log_filter);

	let stderr_logger = fern::Dispatch::new()
		.filter(|md| md.level() == Level::Error || md.level() == Level::Warn)
		.format(|out, message, record| {
			match record.level()
			{
				Level::Error => out.finish(format_args!("{} {}", ERROR_PREFIX.as_str(), message)),
				Level::Warn => out.finish(format_args!("{} {}", WARNING_PREFIX.as_str(), message)),
				_ => (),
			};
		})
		.chain(std::io::stderr());

	let stdout_logger = fern::Dispatch::new()
		.filter(|md| {
			md.level() == Level::Info || md.level() == Level::Debug || md.level() == Level::Trace
		})
		.format(|out, message, record| {
			match record.level()
			{
				Level::Info => out.finish(format_args!("{}", message)),
				Level::Debug => out.finish(format_args!("({}) {}", record.target(), message)),
				Level::Trace =>
				{
					let file: &str = record.file().unwrap_or("<unknown>");
					let line: u32 = record.line().unwrap_or(0);

					out.finish(format_args!(
						"({} {}:{}) {}",
						record.target(),
						file,
						line,
						message
					))
				}
				_ => (),
			};
		})
		.chain(std::io::stdout());

	base_config
		.chain(stderr_logger)
		.chain(stdout_logger)
		.apply()
		.expect("Could not initialise logger");
}

fn print_banner()
{
	let build_id_ptr: *const c_char = Cmds::bspcore_get_build_identifier_string();
	let build_id: &'static CStr = unsafe { CStr::from_ptr(build_id_ptr) };
	let bin_name: String = colorize_string(format!("<b>{}</b>", env!("CARGO_BIN_NAME")));

	info!(
		"\n\
		================================================================================\n\
		{bin_name} version {} ({})\n\
		================================================================================",
		env!("CARGO_PKG_VERSION"),
		build_id.to_str().unwrap()
	);
}
