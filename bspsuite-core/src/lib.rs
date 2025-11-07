// By default, deny usage of unsafe code.
// We will be using unsafe code in places, but this
// should not need to occur outside the cinterface module.
// Adding this check should make it much easier to detect
// if FFI-like code has spilled out into places it shouldn't be.
#![deny(unsafe_code)]

use clap::Parser;

mod cinterface;
mod cli;
mod extinterface;
pub mod model;

fn run_compile_command(args: &cli::CompileCommandArgs) -> Result<(), cli::CommandError>
{
	let output_path: &str = args.output_file.to_str().ok_or(cli::CommandError::new(
		cli::ErrorCode::CommandLineError,
		"Error parsing output file path",
	))?;

	// TODO: Implement this properly.
	println!("Compile command run with output file: {output_path}");
	return Ok(());
}

pub fn run_from_shell_arguments(args: &Vec<String>) -> i32
{
	let parsed_args: cli::Cli = cli::Cli::parse_from(args.iter());

	let subcommand: &cli::Subcommand = &parsed_args.command;
	let result: Result<(), cli::CommandError> = match subcommand
	{
		cli::Subcommand::Compile(args) => run_compile_command(&args),
	};

	let exit_code: i32 = match result
	{
		Ok(()) => 0,
		Err(err) =>
		{
			let error_string: &str = &err.description;
			eprintln!("{subcommand} command failed: {error_string}");

			err.exit_code()
		}
	};

	return exit_code;
}
