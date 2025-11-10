// Let's try and keep this build as clean as possible.
#![deny(unused_variables)]
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;

use anyhow::Context;
use clap::Parser;
use glob;
use glob::Paths;
use glob::PatternError;
use target_lexicon::{HOST, OperatingSystem};

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

	let src_dir: PathBuf = binaries_dir();
	let dist_dir: PathBuf = src_dir.join("dist");

	if dist_dir.is_file()
	{
		Err(format!(
			"Dist directory {} is actually a file",
			dist_dir.to_str().unwrap()
		))?;
	}

	if !dist_dir.is_dir()
	{
		std::fs::create_dir(dist_dir.clone()).with_context(|| {
			format!(
				"Failed to create dist directory {}",
				dist_dir.to_str().unwrap()
			)
		})?;
	}

	copy_files(&src_dir, &dist_dir, library_extension_for_platform())?;

	Ok(())
}

fn copy_files(src: &PathBuf, dest: &PathBuf, extension: &str) -> Result<(), DynError>
{
	if !dest.is_dir()
	{
		Err(format!(
			"Dest directory {} does not exist",
			dest.to_str().unwrap()
		))?;
	}

	let glob_str: String = format!("{}/*{extension}", src.to_str().unwrap());
	let glob_result: Paths = glob::glob(glob_str.as_str()).unwrap();

	for source_file in glob_result
	{
		let file_name: String =
			String::from(source_file.unwrap().file_name().unwrap().to_str().unwrap());

		let source_path: PathBuf = src.join(file_name.clone());
		let dest_path: PathBuf = dest.join(file_name.clone());

		println!(
			"Copying {} to {}",
			source_path.to_str().unwrap(),
			dest_path.to_str().unwrap()
		);

		std::fs::copy(source_path, dest_path)?;
	}

	Ok(())
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

const fn library_extension_for_platform() -> &'static str
{
	return match HOST.operating_system
	{
		OperatingSystem::Windows => ".dll",
		OperatingSystem::Linux => ".so",
		_ => panic!("Unsupported operating system"),
	};
}

const fn executable_extension_for_platform() -> &'static str
{
	return match HOST.operating_system
	{
		OperatingSystem::Windows => ".exe",
		OperatingSystem::Linux => "",
		_ => panic!("Unsupported operating system"),
	};
}
