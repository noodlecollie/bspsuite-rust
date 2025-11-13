use chrono::{DateTime, TimeZone, Utc};
use std::env;
use std::process::Command;

fn main()
{
	let build_date: String = get_build_date_time()
		.format("%Y-%m-%dT%H:%M:%SZ")
		.to_string();

	println!("cargo:rustc-env=BUILD_DATE={build_date}");

	if let Some(git) = get_git_version()
	{
		println!("cargo:rustc-env=VCS_HASH={git}");
	}
}

fn get_build_date_time() -> DateTime<Utc>
{
	return match env::var("SOURCE_DATE_EPOCH")
	{
		Ok(val) => Utc.timestamp_opt(val.parse::<i64>().unwrap(), 0).unwrap(),
		Err(_) => Utc::now(),
	};
}

fn get_git_version() -> Option<String>
{
	let commit_hash = Command::new("git")
		.args(["rev-parse", "--short=10", "HEAD"])
		.output()
		.ok()
		.filter(|output| output.status.success())
		.and_then(|output| {
			let commit_hash = String::from_utf8(output.stdout)
				.ok()
				.map(|s| s.trim().to_string());

			commit_hash
		});

	let is_dirty = Command::new("git")
		.args(["status", "--porcelain"])
		.output()
		.map(|out| !out.stdout.is_empty())
		.unwrap_or(false);

	let dirty_suffix = if is_dirty { "-dirty" } else { "" };
	Some(format!("{}{}", commit_hash?, dirty_suffix))
}
