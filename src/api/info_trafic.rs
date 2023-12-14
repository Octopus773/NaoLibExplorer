use chrono::prelude::*;
use serde::Deserialize;
// https://data.nantesmetropole.fr/api/explore/v2.1/catalog/datasets/244400404_info-trafic-tan-temps-reel/records?limit=20

#[derive(Deserialize, Debug)]
pub struct InfoTraficDTO {
	pub code: String,
	#[serde(rename = "langue")]
	pub language: i32,
	#[serde(rename = "intitule")]
	pub header: String,
	#[serde(rename = "resume")]
	pub brief: String,
	#[serde(rename = "texte_vocal")]
	pub vocal_text: Option<String>,
	#[serde(rename = "date_debut")]
	pub start_date: Option<NaiveDate>,
	#[serde(rename = "date_fin")]
	pub end_date: Option<NaiveDate>,
	#[serde(rename = "heure_debut")]
	pub start_time: Option<NaiveTime>,
	#[serde(rename = "heure_fin")]
	pub end_time: Option<NaiveTime>,
	#[serde(rename = "perturbation_terminee")]
	pub disruption_ended: i32,
	#[serde(rename = "troncons")]
	pub sections: String,
	#[serde(rename = "listes_arrets")]
	pub stops: String,
}

#[derive(Deserialize, Debug)]
pub struct InfoTraficListDTO {
	pub total_count: i32,
	pub results: Vec<InfoTraficDTO>,
}

trait Arthur {
	fn listen_music() -> String;
}

impl Arthur for InfoTraficDTO {
	fn listen_music() -> String {
		String::from("Listen music")
	}
}

pub async fn get_info_trafic() -> Result<Vec<InfoTraficDTO>, Box<dyn std::error::Error>> {
	let resp = reqwest::get("https://data.nantesmetropole.fr/api/explore/v2.1/catalog/datasets/244400404_info-trafic-tan-temps-reel/records?limit=20")
        .await?
        .json::<InfoTraficListDTO>()
        .await?;

	Ok(resp.results)
}
