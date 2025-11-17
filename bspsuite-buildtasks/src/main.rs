// Let's try and keep this build as clean as possible.
#![deny(unused_variables)]
#![deny(dead_code)]

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use anyhow::{Context, Error, bail};
use clap::Parser;
use glob;
use glob::Paths;
use paris::LogIcon;
use target_lexicon::{HOST, OperatingSystem};

// A lot of code in this file is based off
// https://github.com/matklad/cargo-xtask/blob/master/examples/hello-world/xtask/src/main.rs

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
	let result: Result<(), Error> = match subcommand
	{
		Subcommand::Build => run_build_command(),
	};

	if let Err(e) = result
	{
		for item in e.chain()
		{
			eprintln!("{item}");
		}
	};
}

fn run_build_command() -> Result<(), Error>
{
	// Compiler also builds core library.
	build_crate("bspsuite-compiler")?;

	build_extensions()?;

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

fn build_extensions() -> Result<(), Error>
{
	let glob_str: String = format!("{}/bspsuite-ext-*", project_root().to_str().unwrap());
	let glob_result: Paths = glob::glob(glob_str.as_str()).unwrap();

	for path in glob_result
	{
		if let Ok(path) = path
		{
			if path.is_dir()
			{
				build_crate(path.to_str().unwrap())?;
			}
		}
	}

	Ok(())
}

fn build_crate(dir_name: &str) -> Result<(), Error>
{
	let result = run_cargo(&["build"], &project_root().join(dir_name))?;

	if !result.success()
	{
		bail!("Failed to build {dir_name}");
	}

	Ok(())
}

fn run_cargo(args: &[&str], cwd: &PathBuf) -> Result<ExitStatus, std::io::Error>
{
	let cargo: String = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	return Command::new(cargo).current_dir(cwd).args(args).status();
}

fn create_dist_dir(dist_dir: &PathBuf) -> Result<(), Error>
{
	create_dir(&dist_dir)?;
	create_dir(&dist_dir.join("games"))?;
	create_dir(&dist_dir.join("extensions"))?;

	Ok(())
}

fn create_dir(dir: &PathBuf) -> Result<(), Error>
{
	if dir.is_file()
	{
		bail!(
			"Failed to create directory: {} is actually a file",
			dir.to_str().unwrap()
		);
	}

	if !dir.is_dir()
	{
		std::fs::create_dir(dir.clone())
			.with_context(|| format!("Failed to create directory {}", dir.to_str().unwrap()))?;
	}

	Ok(())
}

fn copy_glob(src: &PathBuf, dest: &PathBuf, glob_str: &str) -> Result<(), Error>
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

fn copy_file(src: &PathBuf, dest: &PathBuf, name: &str) -> Result<(), Error>
{
	let source_path: PathBuf = src.join(name);
	let dest_path: PathBuf = dest.join(name);

	if source_path == dest_path
	{
		return Ok(());
	}

	let should_copy = {
		if !dest_path.is_file()
		{
			true
		}
		else
		{
			let source_metadata = source_path.metadata().with_context(|| {
				format!(
					"Failed to get file metadata for {}",
					source_path.to_str().unwrap()
				)
			})?;

			let dest_metadata = dest_path.metadata().with_context(|| {
				format!(
					"Failed to get file metadata for {}",
					dest_path.to_str().unwrap()
				)
			})?;

			source_metadata.modified()? > dest_metadata.modified()?
		}
	};

	if should_copy
	{
		println!(
			"{} {} -> {}",
			LogIcon::Tick,
			source_path.to_str().unwrap(),
			dest_path.to_str().unwrap()
		);

		std::fs::copy(&source_path, &dest_path)
			.with_context(|| format!("Failed to copy {}", source_path.to_str().unwrap()))?;
	}
	else
	{
		println!("• {}", dest_path.to_str().unwrap())
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
