use super::lesson::Lesson;
use super::request_params::{DayOfWeek, Group};

#[derive(Debug, Clone)]
pub enum Entry {
    NoLessons,
    OneLesson(Lesson),
    MultiLessons(Vec<Option<Lesson>>),
    Other(String),
}

#[derive(Debug, Clone)]
pub struct ScheduleTable {
    pub entries: Vec<Entry>, 
    pub replaced: Vec<Option<Entry>>,
}

impl ScheduleTable  {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            replaced: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn add_replaced(&mut self, entry: Option<Entry>) {
        self.replaced.push(entry);
    }
}

pub trait ScheduleTableGenerator {
    fn get_table(&self, group: Group, weekday: DayOfWeek) -> Result<ScheduleTable, ()>;
}

pub trait ScheduleTableFormatter {
    fn format(&self, table: &ScheduleTable) -> String;
}
