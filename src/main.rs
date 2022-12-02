use bastard_backend::Parser;
use bastard_core::{DayOfWeek, Group, Table, TableGenerator};

fn main() {
    let parser = Parser::new("https://lyceum.urfu.ru/".to_string());
    let table = parser.get_table(Group::_11N, DayOfWeek::Monday);
    println!("{:?}", table);
}
