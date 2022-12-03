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
        let new = &self.lessons;

        let mut table = ScheduleTable::new();
        for i in 1..=7 {
            let entry = new.iter().filter(|x| x.number == i).map(|x| x.clone()).collect::<Vec<RawLesson>>(); 
            let count = entry.len();
            let entry = match count {
                0 => {
                    Entry::NoLessons
                },
                1 => {
                    let one = entry[0].clone();
                    let one = Lesson {
                        name: one.subject,
                        teacher: one.teacher,
                        auditory: one.auditory,
                        replacing: None,
                    };
                    Entry::OneLesson(one)
                },
                2 => {
                    let one = entry[0].clone();
                    let one = Lesson {
                        name: one.subject,
                        teacher: one.teacher,
                        auditory: one.auditory,
                        replacing: None,
                    };
                    let two = entry[1].clone();
                    let two = Lesson {
                        name: two.subject,
                        teacher: two.teacher,
                        auditory: two.auditory,
                        replacing: None,
                    };
                    Entry::TwoLessons(Some(one), Some(two))
                },
                _ => unreachable!(),
            };
            table.add_entry(entry);
        }
        return table;
    }
}
