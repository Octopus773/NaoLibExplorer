use crate::naolibexplorer::{GeoStop, Line, LineType};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct LineDTO {
	#[serde(rename = "numLigne")]
	pub num_line: String,
}

#[derive(Deserialize, Debug)]
struct GeoStopDTO {
	#[serde(rename = "codeLieu")]
	pub code_stop: String,
	#[serde(rename = "libelle")]
	pub name: String,
	#[serde(rename = "distance")]
	pub distance: String,
	#[serde(rename = "ligne")]
	pub lines: Vec<LineDTO>,
}

fn to_geo_stop(gs: GeoStopDTO) -> GeoStop {
	GeoStop {
		code_stop: gs.code_stop,
		name: gs.name,
		// distance is a string like "123 m"
		distance: gs
			.distance
			.split(' ')
			.next()
			.unwrap()
			.parse::<u32>()
			.unwrap(),
		lines: gs
			.lines
			.into_iter()
			.map(|l| Line::new(l.num_line, LineType::BUS))
			.collect(),
	}
}

/// Returns a list of near stops (500m) from the given coordinates
///
/// # Arguments
///
/// * `lat` - GPS Latitude
/// * `lon` - GPS Longitude
///
pub async fn get_near_stops(
	lat: f64,
	lon: f64,
) -> Result<Vec<GeoStop>, Box<dyn std::error::Error>> {
	let url = format!("https://open.tan.fr/ewp/arrets.json/{}/{}", lat, lon);
	let resp = reqwest::get(&url)
		.await?
		.json::<Vec<GeoStopDTO>>()
		.await?
		.into_iter()
		.map(to_geo_stop)
		.collect();
	Ok(resp)
}
