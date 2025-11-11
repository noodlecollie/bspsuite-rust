use super::libstate;
use anyhow::Error;

pub fn error_string(err: &Error) -> String
{
	return format!("<b>{err}</>\nCaused by:\n    {}", err.root_cause());
}

pub fn verbose_error_string(err: &Error) -> String
{
	let mut out: String = String::new();

	for (index, item) in err.chain().enumerate()
	{
		if index == 0
		{
			out.push_str(format!("<b>[*] {item}</>").as_str());
		}
		else
		{
			out.push_str(format!("\n[+] {item}").as_str());
		}
	}

	return out;
}

pub fn log_string(err: &Error) -> String
{
	return if libstate::get().verbose()
	{
		verbose_error_string(err)
	}
	else
	{
		error_string(err)
	};
}
