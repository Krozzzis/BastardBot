use bastard_core::schedule_table::{ScheduleTable, ScheduleTableFormatter, Entry};
use bastard_core::lesson::{Lesson, LessonFormatter};

pub struct SimpleFormatter {
    lesson_formatter: Box<dyn LessonFormatter>,
}
pub struct SimpleLessonFormatter {}

impl LessonFormatter for SimpleLessonFormatter {
    fn format_lesson(&self, lesson: Option<&Lesson>) -> String {
        let mut output = String::new();

        if let Some(lesson) = lesson {
            if !lesson.name.is_empty() {
                output += lesson.name.as_str();
            } else {
                output += "-";
            }

            if !lesson.auditory.is_empty() {
                output += format!("[{}], ", lesson.auditory).as_str();
            } else {
                output += "[-], ";
            }

            if !lesson.teacher.is_empty() {
                output += lesson.teacher.as_str();
            } else {
                output += "-";
            }
        } else {
            output += "Нет";
        }

        output
    }
}

impl SimpleFormatter {
    pub fn new() -> Self {
        Self {
            lesson_formatter: Box::new(SimpleLessonFormatter{})
        }
    }

    fn format_line(&self, indentation: &str, changed: &str, lesson: Option<&Lesson>) -> String {
        format!("{} {}{}\n", indentation, changed, self.lesson_formatter.format_lesson(lesson))
    }
}

impl ScheduleTableFormatter for SimpleFormatter {
    fn format(&self, table: &ScheduleTable) -> String {
        let mut output = String::new();

        for (i, entry) in (&table.entries).iter().enumerate() {
            output += format!("{} Урок:\n", i+1).as_str();
            let changed_char = if let None = table.replaced[i] {
                " "
            } else {
                "*"
            };
            match entry {
                Entry::NoLessons => {
                    output += self.format_line(" ", changed_char, None).as_str();
                },
                Entry::OneLesson(lesson) => {
                    output += self.format_line("└", changed_char, Some(&lesson)).as_str();
                },
                Entry::MultiLessons(list) => {
                    let last_pos = list.len() - 1;
                    for (pos, lesson) in list.iter().enumerate() {
                        let indentation = if pos == last_pos {
                            "└"
                        } else {
                            "├"
                        };

                        if let Some(a) = lesson {
                            output += self.format_line(indentation, changed_char, Some(&a)).as_str();
                        } else {
                            output += self.format_line(indentation, changed_char, None).as_str();
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
