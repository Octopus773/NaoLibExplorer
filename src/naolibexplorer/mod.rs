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
    pub display_time_fr: String,
    pub last_departure: bool,
    pub is_real_time: bool,
    pub line: Line,
}
