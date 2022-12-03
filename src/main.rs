use bastard_backend::WebParser;
use bastard_frontend::SimpleFormatter;
use bastard_core::request_params::{DayOfWeek, Group};
use bastard_core::schedule_table::{ScheduleTableGenerator, ScheduleTableFormatter};

fn main() {
    let parser = WebParser::new("https://lyceum.urfu.ru/".to_string());
    let table = parser.get_table(Group::_11N, DayOfWeek::Saturday).expect("Ooh, error");
    let formatter = SimpleFormatter::new();
    let output = formatter.format(&table);
    println!("{}", output);
}
