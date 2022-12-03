use serde::{Serialize, Deserialize};
use bastard_core::schedule_table::{ScheduleTable, Entry};
use bastard_core::lesson::Lesson;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawLesson {
    subject: String,
    auditory: String,
    teacher: String,
    subgroup: i32,
    number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTable {
    lessons: Vec<RawLesson>,
    //diffs: Vec<RawLesson>,
}

impl RawTable {
    pub fn to_table(&self) -> ScheduleTable {
        let mut table = ScheduleTable::new();

        if self.lessons.len() == 0 {
            return table;
        }
        let min_place: i32 = self.lessons.iter().min_by(|a, b| a.number.cmp(&b.number)).unwrap().number;
        let max_place: i32 = self.lessons.iter().max_by(|a, b| a.number.cmp(&b.number)).unwrap().number;
        for place in min_place..=max_place {
            let lessons: Vec<&RawLesson> = self.lessons
                .iter()
                .filter(|x| x.number == place)
                .collect();

            let count = lessons.len();
            
            let entry = match count {
                0 => Entry::NoLessons,
                1 => {
                    let lesson = lessons[0];
                    let one = Lesson {
                        name: lesson.subject.clone(),
                        teacher: lesson.teacher.clone(),
                        auditory: lesson.auditory.clone(),
                        replacing: None,
                    };
                    match lesson.subgroup {
                        1 => Entry::TwoLessons(Some(one), None),
                        2 => Entry::TwoLessons(None, Some(one)),
                        _ => Entry::OneLesson(one),
                    }
                },
                2 => {
                    let lesson = lessons[0];
                    let one = Lesson {
                        name: lesson.subject.clone(),
                        teacher: lesson.teacher.clone(),
                        auditory: lesson.auditory.clone(),
                        replacing: None,
                    };

                    let lesson = lessons[1];
                    let two = Lesson {
                        name: lesson.subject.clone(),
                        teacher: lesson.teacher.clone(),
                        auditory: lesson.auditory.clone(),
                        replacing: None,
                    };

                    if lessons[0].subgroup > lessons[1].subgroup {
                        Entry::TwoLessons(Some(two), Some(one))
                    } else {
                        Entry::TwoLessons(Some(one), Some(two))
                    }
                },
                _ => unreachable!(),
            };

            table.add_entry(entry);
        }

        return table;
    }
}
