use bastard_core::schedule_table::{ScheduleTable, ScheduleTableFormatter, Entry};
use bastard_core::lesson::{Lesson, LessonFormatter};
use bastard_core::request_params::Group;
use super::meal_schedule::{MealSchedule, MealTime};

pub struct SimpleFormatter {
    lesson_formatter: Box<dyn LessonFormatter>,
    timetable: Option<Vec<(String, String)>>,
    meal_schedule: Option<MealSchedule>,
    group: Group,
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
    pub fn new(timetable: Option<Vec<(String, String)>>, meal_schedule: Option<MealSchedule>, group: Group) -> Self {
        Self {
            lesson_formatter: Box::new(SimpleLessonFormatter{}),
            timetable: timetable,
            meal_schedule: meal_schedule,
            group: group,
        }
    }

    fn format_line(&self, indentation: &str, changed: &str, lesson: Option<&Lesson>) -> String {
        format!("{} {}{}\n", indentation, changed, self.lesson_formatter.format_lesson(lesson))
    }

    fn format_timetable(&self, place: usize) -> Option<String> {
        if let Some(timetable) = &self.timetable {
            if let Some((a, b)) = timetable.get(place) {
                Some(format!("[{}-{}]", a, b))
            } else {
                Some(String::from(""))
            }
        } else {
            None
        }
    }
}

impl ScheduleTableFormatter for SimpleFormatter {
    fn format(&self, table: &ScheduleTable) -> String {
        let mut output = String::new();

        for (i, entry) in (&table.entries).iter().enumerate() {
            if let Some(time) = self.format_timetable(i) {
                output += format!("{} Урок{}:\n", i+1, time.as_str()).as_str();
            } else {
                output += format!("{} Урок:\n", i+1).as_str();
            }
            let changed_char = match table.replaced.get(i) {
                Some(a) => match a {
                    Some(_) => "*",
                    None => " ",
                },
                None => " ",
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
            }
            if let Some(meal_schedule) = &self.meal_schedule {
                if let Some(meals) = meal_schedule.by_group(self.group) {
                    for meal in meals.iter().filter(|x| x.place == i+1) {
                        output += format!("--- {} ---\n", meal.kind).as_str();
                    }
                }
            }
                
        }

        return output;
    }
}
