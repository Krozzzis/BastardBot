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
            let changed: bool = if let None = table.replaced[i] {
                false
            } else {
                true
            };
            match entry {
                Entry::NoLessons => {
                    output += format!(" {}Нет\n", if changed {"*"} else {" "}).as_str();
                },
                Entry::OneLesson(lesson) => {
                    output += format!("└ {}{}[{}] - {}\n", if changed {"*"} else {" "}, lesson.name, lesson.auditory, lesson.teacher).as_str();
                },
                Entry::MultiLessons(list) => {
                    let last_pos = list.len() - 1;
                    for (pos, lesson) in list.iter().enumerate() {
                        if pos == last_pos {
                            output += "└ ";
                        } else {
                            output += "├ ";
                        }
                        if let Some(a) = lesson {
                            output += format!(" {}{}[{}] - {}\n", if changed {"*"} else {" "}, a.name, a.auditory, a.teacher).as_str();
                        } else {
                            output += format!(" {}Нет\n", if changed {"*"} else {" "}).as_str();
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
