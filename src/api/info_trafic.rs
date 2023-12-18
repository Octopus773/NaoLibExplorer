use std::error::Error;

use crate::naolibexplorer::{InfoTrafic, InfoTraficStop, Section};
use chrono::prelude::*;
use serde::Deserialize;
use serde_json::Value;
// https://data.nantesmetropole.fr/api/explore/v2.1/catalog/datasets/244400404_info-trafic-tan-temps-reel/records?limit=20

#[derive(Deserialize, Debug)]
struct InfoTraficDTO {
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
struct InfoTraficListDTO {
	pub total_count: i32,
	pub results: Vec<InfoTraficDTO>,
}

#[derive(Deserialize, Debug)]
struct InfoTrafficStopDTO {
	#[serde(rename = "LIGNE")]
	pub line: String,
	#[serde(rename = "SENS")]
	pub direction: String,
	#[serde(rename = "CODES")]
	pub codes: String,
}

fn to_info_traffic_stop(its_dto: InfoTrafficStopDTO) -> Result<InfoTraficStop, Box<dyn Error>> {
	Ok(InfoTraficStop {
		num_line: its_dto.line,
		direction: its_dto.direction.parse()?,
		codes: its_dto.codes.split(',').map(|s| s.to_owned()).collect(),
	})
}

fn to_info_trafic(s: InfoTraficDTO) -> Result<InfoTrafic, Box<dyn Error>> {
	#[derive(Deserialize)]
	struct Tmp {
		#[serde(rename = "LISTE_ARRETS")]
		sl: Value,
	}

	let stops_data_obj: Tmp = serde_json::from_str(&s.stops)?;

	let stop_data: Vec<InfoTraficStop> = 'sdb: {
		if stops_data_obj.sl.is_string() {
			// here we assume that the string is empty
			break 'sdb vec![];
		}
		if stops_data_obj.sl.is_object() {
			let s: InfoTrafficStopDTO = serde_json::from_value(stops_data_obj.sl)?;
			break 'sdb vec![to_info_traffic_stop(s)?];
		}
		if stops_data_obj.sl.is_array() {
			let s: Vec<InfoTrafficStopDTO> = serde_json::from_value(stops_data_obj.sl)?;
			let res = s
				.into_iter()
				.map(to_info_traffic_stop)
				.collect::<Result<_, _>>();
			match res {
				Ok(v) => break 'sdb v,
				Err(e) => return Err(e),
			}
		}

		return Err(Box::from("LISTE_ARRETS is of unknown type"));
	};

	Ok(InfoTrafic {
		code: s.code,
		language: s.language,
		header: s.header,
		brief: s.brief,
		vocal_text: s.vocal_text,
		start_date: s.start_date,
		end_date: s.end_date,
		start_time: s.start_time,
		end_time: s.end_time,
		disruption_ended: s.disruption_ended,
		sections: s
			.sections
			.split(';')
			.map(|s| Section {
				raw_data: s.to_owned(),
			})
			.collect(),
		stops: stop_data,
	})
}

pub async fn get_info_trafic() -> Result<Vec<InfoTrafic>, Box<dyn std::error::Error>> {
	let resp = reqwest::get("https://data.nantesmetropole.fr/api/explore/v2.1/catalog/datasets/244400404_info-trafic-tan-temps-reel/records?limit=20")
        .await?
        .json::<InfoTraficListDTO>()
        .await?
		.results
		.into_iter()
		.map(to_info_trafic)
		.collect::<Result<_, _>>();

	resp
}
