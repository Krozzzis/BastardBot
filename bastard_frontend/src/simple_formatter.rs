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
                Entry::MultiLessons(list) => {
                    let last_pos = list.len() - 1;
                    for (pos, lesson) in list.iter().enumerate() {
                        if pos == 0 {
                            output += "├ ";
                        } else if pos == last_pos {
                            output += "└ ";
                        } else {
                            output += "├ ";
                        }
                        if let Some(a) = lesson {
                            output += format!("{}[{}] - {}\n", a.name, a.auditory, a.teacher).as_str();
                        } else {
                            output += "Нет\n";
                        }
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
