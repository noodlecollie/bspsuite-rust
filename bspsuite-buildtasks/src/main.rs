// Let's try and keep this build as clean as possible.
#![deny(unused_variables)]
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;

use clap::Parser;

// A lot of code in this file is based off
// https://github.com/matklad/cargo-xtask/blob/master/examples/hello-world/xtask/src/main.rs

type DynError = Box<dyn std::error::Error>;

/// Commands that may be executed on the compiler executable.
#[derive(clap::Parser)]
#[command(version, about = "BSPSuite build task runner", long_about = None)]
pub struct Cli
{
	#[command(subcommand)]
	pub command: Subcommand,
}

#[derive(clap::Subcommand, strum::Display)]
pub enum Subcommand
{
	/// Build the compiler toolchain and copy it
	/// into a canonical directory structure.
	Build,
}

fn main()
{
	let parsed_args: Cli = Cli::parse();
	let subcommand: &Subcommand = &parsed_args.command;
	let result: Result<(), DynError> = match subcommand
	{
		Subcommand::Build => run_build_command(),
	};

	if let Err(e) = result
	{
		eprintln!("{}", e);
	};
}

fn run_build_command() -> Result<(), DynError>
{
	let cargo: String = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

	let status: ExitStatus = Command::new(cargo)
		.current_dir(project_root())
		.args(&["build"])
		.status()?;

	if !status.success()
	{
		Err("cargo build failed")?;
	}

	return Ok(());
}

fn project_root() -> PathBuf
{
	Path::new(&env!("CARGO_MANIFEST_DIR"))
		.ancestors()
		.nth(1)
		.unwrap()
		.to_path_buf()
}

fn binaries_dir() -> PathBuf
{
	Path::new(&env!("OUT_DIR"))
		.ancestors()
		.nth(3)
		.unwrap()
		.to_path_buf()
}
