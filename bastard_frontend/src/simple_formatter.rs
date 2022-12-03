use bastard_core::schedule_table::{ScheduleTable, ScheduleTableFormatter, Entry};

pub struct SimpleFormatter {}

impl SimpleFormatter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ScheduleTableFormatter for SimpleFormatter {
    fn format(&self, table: &ScheduleTable) -> String {
        let mut output = String::new();

        for (i, entry) in (&table.entries).iter().enumerate() {
            output += format!("{} Урок:\n", i+1).as_str();
            match entry {
                Entry::NoLessons => {
                    output += "Нет\n";
                },
                Entry::OneLesson(lesson) => {
                    output += format!("{}[{}] - {}\n", lesson.name, lesson.auditory, lesson.teacher).as_str();
                },
                Entry::TwoLessons(one, two) => {
                    if let Some(a) = one {
                        output += format!("{}[{}] - {}\n", a.name, a.auditory, a.teacher).as_str();
                    } else {
                        output += "Нет\n";
                    }

                    if let Some(a) = two {
                        output += format!("{}[{}] - {}\n", a.name, a.auditory, a.teacher).as_str();
                    } else {
                        output += "Нет\n";
                    }
                },
                Entry::Other(text) => {
                    output += text;
                    output += "\n";
                }
            }
        }

        return output;
    }
}
