use super::types::{DayOfWeek, Group, Table};

pub trait TableGenerator {
    fn get_table(&self, group: Group, weekday: DayOfWeek) -> Result<Table, ()>;
}

pub trait TableFormatter {
    fn format(&self, table: &Table) -> String;
}
