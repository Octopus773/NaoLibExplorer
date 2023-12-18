use crate::naolibexplorer::WaitingTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct LineDTO {
	#[serde(rename = "numLigne")]
	num_line: String,
	#[serde(rename = "typeLigne")]
	line_type: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopDTO {
	#[serde(rename = "codeArret")]
	code_stop: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct WaitingTimeDTO {
	pub sens: i32,
	pub terminus: String,
	#[serde(rename = "infotrafic")]
	pub info_trafic: bool,
	pub temps: String,
	#[serde(rename = "dernierDepart")]
	pub last_departure: String,
	#[serde(rename = "tempsReel")]
	pub is_real_time: String,
	pub ligne: LineDTO,
	#[serde(rename = "arret")]
	pub stop: StopDTO,
}

fn to_waiting_time(wt: WaitingTimeDTO) -> WaitingTime {
	WaitingTime {
		direction: wt.sens,
		terminus: wt.terminus,
		info_trafic: wt.info_trafic,
		display_time_fr: if !wt.temps.is_empty() {
			Some(wt.temps)
		} else {
			None
		},
		last_departure: wt.last_departure.parse::<bool>().unwrap(),
		is_real_time: wt.is_real_time.parse::<bool>().unwrap(),
		code_stop: wt.stop.code_stop,
		num_line: wt.ligne.num_line,
		line_type: match wt.ligne.line_type {
			1 => crate::naolibexplorer::LineType::TRAMWAY,
			2 => crate::naolibexplorer::LineType::BUSWAY,
			3 => crate::naolibexplorer::LineType::BUS,
			4 => crate::naolibexplorer::LineType::NAVIBUS,
			_ => panic!("Unknown line type"),
		},
	}
}

pub async fn get_waiting_times() -> Result<Vec<WaitingTime>, Box<dyn std::error::Error>> {
	let resp = reqwest::get("https://open.tan.fr/ewp/tempsattente.json/CRQU")
		.await?
		.json::<Vec<WaitingTimeDTO>>()
		.await?
		.into_iter()
		.map(to_waiting_time)
		.collect();
	Ok(resp)
}
