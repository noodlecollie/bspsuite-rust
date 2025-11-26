use crate::compiler_error::{CompilerError, CompilerErrorCode};
use crate::model::MapBlueprint;
use anyhow::Result;
use std::path::PathBuf;

pub fn create_map_blueprint(input_file: std::path::PathBuf) -> Result<MapBlueprint, CompilerError>
{
	// TODO
	return Err(CompilerError::new(
		CompilerErrorCode::InternalError,
		String::from("Not yet implemented"),
	));
}
