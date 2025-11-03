// By default, deny usage of unsafe code.
// We will be using unsafe code in places, but this
// should not need to occur outside the cinterface module.
// Adding this check should make it much easier to detect
// if FFI-like code has spilled out into places it shouldn't be.
#![deny(unsafe_code)]

use clap::Parser;

mod cinterface;
mod cli;

pub fn run_from_shell_arguments(args: &Vec<String>) -> i32
{
	let parsed_args: cli::Cli = cli::Cli::parse_from(args.iter());

	match &parsed_args.command
	{
		cli::Subcommand::Compile { output_file } =>
		{
			// TODO
			let output_path: &str = output_file.to_str().unwrap();
			println!("Compile command run with output file: {output_path}");
			return cli::ErrorCode::exit_code(&cli::ErrorCode::InternalError);
		}
	}
}
