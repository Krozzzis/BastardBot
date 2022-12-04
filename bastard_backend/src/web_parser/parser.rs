use bastard_core::request_params::{DayOfWeek, Group};
use bastard_core::schedule_table::{ScheduleTable, ScheduleTableGenerator};
use super::raw_table::RawTable;

pub struct WebParser {
    url: String,
}

impl WebParser {
    pub fn new(url: String) -> Self {
        Self {url}
    }

    fn get_request_url(&self, group: Group, weekday: DayOfWeek) -> String {
        format!("{}?type=11&scheduleType=group&weekday={}&group={}", self.url, weekday as i32, group as i32)
    }
}

impl ScheduleTableGenerator for WebParser {
    fn get_table(&self, group: Group, weekday: DayOfWeek) -> Result<ScheduleTable, String> {
        let url = self.get_request_url(group, weekday);
        let body = match reqwest::blocking::get(&url) {
            Ok(x) => {
                match x.text() {
                    Ok(a) => a,
                    Err(_) => return Err(String::from("Fetch error")),
                }
            },
            Err(_) => return Err(String::from("Fetch error"))
        };
        let raw_table: RawTable = match serde_json::from_str(&body) {
            Ok(a) => a,
            Err(_) => return Err(String::from("Parsing error")),
        };
        Ok(raw_table.to_table()?)
    }
}
