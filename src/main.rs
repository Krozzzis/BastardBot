use bastard_backend::WebParser;
use bastard_frontend::SimpleFormatter;
use bastard_core::request_params::{DayOfWeek, Group};
use bastard_core::schedule_table::{ScheduleTableGenerator, ScheduleTableFormatter};

fn main() {
    let parser = WebParser::new("https://lyceum.urfu.ru/".to_string());
    let table = parser.get_table(Group::_11N, DayOfWeek::Saturday).expect("Ooh, error");
    let formatter = SimpleFormatter::new(
        vec![
            (String::from("9:00"), String::from("9:40")),
            (String::from("9:50"), String::from("10:30")),
            (String::from("10:45"), String::from("11:25")),
            (String::from("11:40"), String::from("12:20")),
            (String::from("12:35"), String::from("13:15")),
            (String::from("13:35"), String::from("14:15")),
            (String::from("14:35"), String::from("15:15")),
        ]);
    let output = formatter.format(&table);
    println!("{}", output);
}
