use const_cstr::{ConstCStr, const_cstr};
use constcat::concat;

mod compiler_state;
mod model;

pub mod commands;
pub mod extensions;

pub static BUILD_IDENTIFIER: ConstCStr =
	const_cstr!(concat!(env!("BUILD_DATE"), " ", env!("VCS_HASH")));
