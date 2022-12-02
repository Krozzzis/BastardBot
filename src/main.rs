use bastard_backend::Parser;
use bastard_frontend::SimpleFormatter;
use bastard_core::{DayOfWeek, Group, TableGenerator, TableFormatter};

fn main() {
    let parser = Parser::new("https://lyceum.urfu.ru/".to_string());
    let table = parser.get_table(Group::_11N, DayOfWeek::Thursday).expect("Ooh, error");
    let formatter = SimpleFormatter::new();
    let output = formatter.format(&table);
    println!("{}", output);
}
