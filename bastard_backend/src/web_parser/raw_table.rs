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
                        1 => Entry::MultiLessons(vec![Some(one), None]),
                        2 => Entry::MultiLessons(vec![None, Some(one)]),
                        _ => Entry::OneLesson(one),
                    }
                },
                _ => {
                    let mut list: Vec<Option<Lesson>> = Vec::new();
                    let max_subgroup: usize = lessons.iter().max_by(|a, b| a.subgroup.cmp(&b.subgroup)).unwrap().subgroup as usize;

                    for subgroup in 1..=max_subgroup {
                        if let Some(lesson) = lessons.get(subgroup) {
                            let one = Lesson {
                                name: lesson.subject.clone(),
                                teacher: lesson.teacher.clone(),
                                auditory: lesson.auditory.clone(),
                                replacing: None,
                            };
                            list.push(Some(one));
                        } else {
                            list.push(None);
                        }
                    }
                    Entry::MultiLessons(list)
                },
            };

            table.add_entry(entry);
        }

        return table;
    }
}
