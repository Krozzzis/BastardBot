use serde::{Deserialize, Serialize};
use bastard_core::*;

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
    pub fn to_table(&self) -> Table {
        let mut new = self.lessons.clone();

        let mut table = Table::new();
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
                    };
                    Entry::OneLesson(one)
                },
                2 => {
                    let one = entry[0].clone();
                    let one = Lesson {
                        name: one.subject,
                        teacher: one.teacher,
                        auditory: one.auditory,
                    };
                    let two = entry[1].clone();
                    let two = Lesson {
                        name: two.subject,
                        teacher: two.teacher,
                        auditory: two.auditory,
                    };
                    Entry::TwoLessons(Some(one), Some(two))
                },
                _ => unreachable!(),
            };
            table.add_section(Section {entries: vec![entry]});
        }
        return table;
    }
}
