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

	create_dist_dir(&dist_dir)?;

	let lib_prefix: &str = library_prefix_for_platform();
	let lib_ext: &str = library_extension_for_platform();
	let exe_ext: &str = executable_extension_for_platform();

	copy_file(&src_dir, &dist_dir, format!("bspc{exe_ext}").as_str())?;
	copy_file(
		&src_dir,
		&dist_dir,
		format!("{lib_prefix}bspcore{lib_ext}").as_str(),
	)?;

	let glob_str: String = format!("{lib_prefix}*ext{lib_ext}");
	copy_glob(&src_dir, &dist_dir.join("extensions"), glob_str.as_str())?;

	Ok(())
}

fn create_dist_dir(dist_dir: &PathBuf) -> Result<(), DynError>
{
	create_dir(&dist_dir)?;
	create_dir(&dist_dir.join("games"))?;
	create_dir(&dist_dir.join("extensions"))?;

	Ok(())
}

fn create_dir(dir: &PathBuf) -> Result<(), DynError>
{
	if dir.is_file()
	{
		Err(format!(
			"Failed to create directory: {} is actually a file",
			dir.to_str().unwrap()
		))?;
	}

	if !dir.is_dir()
	{
		std::fs::create_dir(dir.clone())
			.with_context(|| format!("Failed to create directory {}", dir.to_str().unwrap()))?;
	}

	Ok(())
}

fn copy_glob(src: &PathBuf, dest: &PathBuf, glob_str: &str) -> Result<(), DynError>
{
	let full_glob_str: String = format!("{}/{glob_str}", src.to_str().unwrap());
	let glob_result: Paths = glob::glob(full_glob_str.as_str()).unwrap();

	for source_file in glob_result
	{
		copy_file(
			src,
			dest,
			source_file.unwrap().file_name().unwrap().to_str().unwrap(),
		)?;
	}

	Ok(())
}

fn copy_file(src: &PathBuf, dest: &PathBuf, name: &str) -> Result<(), DynError>
{
	let source_path: PathBuf = src.join(name);
	let dest_path: PathBuf = dest.join(name);

	if source_path == dest_path
	{
		return Ok(());
	}

	println!(
		"Copying: {} -> {}",
		source_path.to_str().unwrap(),
		dest_path.to_str().unwrap()
	);

	std::fs::copy(&source_path, &dest_path)
		.with_context(|| format!("Failed to copy {}", source_path.to_str().unwrap()))?;

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

const fn library_prefix_for_platform() -> &'static str
{
	return match HOST.operating_system
	{
		OperatingSystem::Windows => "",
		OperatingSystem::Linux => "lib",
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
