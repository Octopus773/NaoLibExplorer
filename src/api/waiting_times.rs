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
struct ArretDTO {
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
}

impl WaitingTimeDTO {
    pub fn to_waiting_time(&self) -> WaitingTime {
        WaitingTime {
            direction: self.sens,
            terminus: self.terminus.clone(),
            info_trafic: self.info_trafic,
            display_time_fr: self.temps.clone(),
            last_departure: self.last_departure.parse::<bool>().unwrap(),
            is_real_time: self.is_real_time.parse::<bool>().unwrap(),
            line: match self.ligne.line_type {
                1 => crate::naolibexplorer::Line {
                    num_line: self.ligne.num_line.clone(),
                    line_type: crate::naolibexplorer::LineType::TRAMWAY,
                    directions: None,
                    accessible: None,
                    traffic_status: None,
                },
                2 => crate::naolibexplorer::Line {
                    num_line: self.ligne.num_line.clone(),
                    line_type: crate::naolibexplorer::LineType::BUS,
                    directions: None,
                    accessible: None,
                    traffic_status: None,
                },
                3 => crate::naolibexplorer::Line {
                    num_line: self.ligne.num_line.clone(),
                    line_type: crate::naolibexplorer::LineType::BUSWAY,
                    directions: None,
                    accessible: None,
                    traffic_status: None,
                },
                _ => crate::naolibexplorer::Line {
                    num_line: self.ligne.num_line.clone(),
                    line_type: crate::naolibexplorer::LineType::BUS,
                    directions: None,
                    accessible: None,
                    traffic_status: None,
                },
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
