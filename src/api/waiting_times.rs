use crate::naolibexplorer::WaitingTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Line {
	TRAMWAY(String),
	BUS(String),
	BUSWAY(String),
}

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

impl WaitingTimeDTO {
	pub fn to_waiting_time(&self) -> WaitingTime {
		WaitingTime {
			direction: self.sens,
			terminus: self.terminus.clone(),
			info_trafic: self.info_trafic,
			display_time_fr: if !self.temps.is_empty() {
				Some(self.temps.clone())
			} else {
				None
			},
			last_departure: self.last_departure.parse::<bool>().unwrap(),
			is_real_time: self.is_real_time.parse::<bool>().unwrap(),
			code_stop: self.stop.code_stop.clone(),
			num_line: self.ligne.num_line.clone(),
			line_type: match self.ligne.line_type {
				1 => crate::naolibexplorer::LineType::TRAMWAY,
				2 => crate::naolibexplorer::LineType::BUS,
				3 => crate::naolibexplorer::LineType::BUSWAY,
				_ => panic!("Unknown line type"),
			},
		}
	}
}

pub async fn get_waiting_times() -> Result<Vec<WaitingTime>, Box<dyn std::error::Error>> {
	let resp = reqwest::get("https://open.tan.fr/ewp/tempsattente.json/CRQU")
		.await?
		.json::<Vec<WaitingTimeDTO>>()
		.await?
		.iter()
		.map(|x| x.to_waiting_time())
		.collect();
	Ok(resp)
}
