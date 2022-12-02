use bastard_core::TableGenerator;
use bastard_core::{DayOfWeek, Group, Table};
use super::RawTable;

pub struct Parser {
    url: String,
}

impl Parser {
    pub fn new(url: String) -> Self {
        Self {url}
    }

    fn get_request_url(&self, group: Group, weekday: DayOfWeek) -> String {
        format!("{}?type=11&scheduleType=group&weekday={}&group={}", self.url, weekday as i32, group as i32)
    }
}

impl TableGenerator for Parser {
    fn get_table(&self, group: Group, weekday: DayOfWeek) -> Result<Table, ()> {
        let url = self.get_request_url(group, weekday);
        println!("{}", url);
        let body = reqwest::blocking::get(&url).expect("Oops, get error").text().unwrap();
        let raw_table: RawTable = serde_json::from_str(&body).expect("Oops, parse error");
        Ok(raw_table.to_table())
    }
}
