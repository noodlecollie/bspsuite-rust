use std::path::PathBuf;

#[derive(Copy, Clone, Debug, strum::Display, clap::ValueEnum)]
pub enum DebugLevel
{
	/// No debug logging will occur.
	Off,

	/// Normal debug logs will be printed.
	On,

	/// Debug and trace logs will be printed.
	Trace,
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli
{
	#[arg(short, long)]
	pub debug: Option<DebugLevel>,

	#[command(subcommand)]
	pub command: Subcommand,
}

#[derive(clap::Subcommand, strum::Display)]
pub enum Subcommand
{
	/// Compile a map from a source file.
	Compile(CompileCommandArgs),
}

#[derive(clap::Args)]
pub struct CompileCommandArgs
{
	/// Path to output file. If this exists, it will be overwritten.
	#[arg(short, long)]
	pub output_file: PathBuf,
}
