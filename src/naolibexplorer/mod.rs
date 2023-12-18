use chrono::{NaiveDate, NaiveTime};

#[derive(Debug)]
pub enum LineType {
	TRAMWAY,
	BUS,
	BUSWAY,
}

#[derive(Debug)]
pub struct Line {
	pub num_line: String,
	pub line_type: LineType,
	pub directions: Option<[String; 2]>,
	pub accessible: Option<bool>,
	pub traffic_status: Option<i32>,
}

#[derive(Debug)]
struct Stop {
	pub code_stop: String,
	pub name: String,
	pub lines: Vec<Line>,
}

#[derive(Debug)]
pub struct WaitingTime {
	pub direction: i32,
	pub terminus: String,
	pub info_trafic: bool,
	pub display_time_fr: Option<String>,
	pub last_departure: bool,
	pub is_real_time: bool,
	pub num_line: String,
	pub line_type: LineType,
	pub code_stop: String,
}

#[derive(Debug)]
pub struct Section {
	// don't know yet how to represent this
	pub raw_data: String,
}

#[derive(Debug)]
pub struct InfoTraficStop {
	pub num_line: String,
	pub direction: i32,
	pub codes: Vec<String>,
}

#[derive(Debug)]
pub struct InfoTrafic {
	pub code: String,
	pub language: i32,
	pub header: String,
	pub brief: String,
	pub vocal_text: Option<String>,
	pub start_date: Option<NaiveDate>,
	pub end_date: Option<NaiveDate>,
	pub start_time: Option<NaiveTime>,
	pub end_time: Option<NaiveTime>,
	pub disruption_ended: i32,
	pub sections: Vec<Section>,
	pub stops: Vec<InfoTraficStop>,
}
