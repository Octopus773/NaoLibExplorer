use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Line {
    TRAMWAY(String),
    BUS(String),
    BUSWAY(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaitingTimeDTO {
    #[serde(rename = "sens")]
    direction: i32,
    terminus: String,
    #[serde(rename = "infotrafic")]
    info_trafic: bool,
    temps: String,
    #[serde(rename = "dernierDepart")]
    last_departure: String,
    #[serde(rename = "tempsReel")]
    is_real_time: String,
    // line: Line,
}

pub async fn get_waiting_times() -> Result<Vec<WaitingTimeDTO>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://open.tan.fr/ewp/tempsattente.json/CRQU")
        .await?
        .json::<Vec<WaitingTimeDTO>>()
        .await
        .expect("Failed to parse waiting times");
    println!("{:#?}", resp);
    Ok(resp)
}
