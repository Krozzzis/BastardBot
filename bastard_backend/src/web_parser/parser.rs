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
    fn get_table(&self, group: Group, weekday: DayOfWeek) -> Result<ScheduleTable, ()> {
        let url = self.get_request_url(group, weekday);
        let body = reqwest::blocking::get(&url).expect("Oops, get error").text().unwrap();
        let raw_table: RawTable = serde_json::from_str(&body).expect("Oops, parse error");
        Ok(raw_table.to_table())
    }
}
