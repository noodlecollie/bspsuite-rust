use std::collections::HashMap;

use super::dplane3::DPlane3;

pub struct BlueprintBrushFace
{
	pub plane: DPlane3,
	pub material: String,
}

pub struct BlueprintBrush
{
	pub faces: Vec<BlueprintBrushFace>,
}

pub struct BlueprintEntity
{
	pub brushes: Vec<BlueprintBrush>,
	pub keyvalues: HashMap<String, String>,
}

pub struct MapBlueprint
{
	pub entities: Vec<BlueprintEntity>,
}
