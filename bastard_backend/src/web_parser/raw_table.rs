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

impl RawLesson {
    pub fn to_lesson(&self) -> Lesson {
        Lesson {
            name: self.subject.clone(),
            auditory: self.auditory.clone(),
            teacher: self.teacher.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTable {
    lessons: Vec<RawLesson>,
    diffs: Vec<RawLesson>,
}

impl RawTable {
    fn get_entry(&self, lessons: &Vec<&RawLesson>) -> Entry {
        let count = lessons.len();
        let entry: Entry = match count {
            0 => Entry::NoLessons,
            1 => {
                let lesson = lessons[0];
                let one = (*lesson).to_lesson();
                match lesson.subgroup {
                    0 => Entry::OneLesson(one),
                    1 => Entry::MultiLessons(vec![Some(one), None]),
                    _ => Entry::MultiLessons(vec![None, Some(one)]),
                }
            },
            _ => {
                let mut list: Vec<Option<Lesson>> = Vec::new();
                let max_subgroup: usize = lessons.iter().max_by(|a, b| a.subgroup.cmp(&b.subgroup)).unwrap().subgroup as usize;

                for subgroup in 1..=max_subgroup {
                    if let Some(lesson) = lessons.get(subgroup - 1) {
                        let one = (*lesson).to_lesson();
                        list.push(Some(one));
                    } else {
                        list.push(None);
                    }
                }
                Entry::MultiLessons(list)
            },

        };
        return entry;
    }

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
            let entry = self.get_entry(&lessons);

            let diffs: Vec<&RawLesson> = self.diffs
                .iter()
                .filter(|x| x.number == place)
                .collect();
            let diff = self.get_entry(&diffs);

            if let Entry::NoLessons = diff {
                table.add_entry(entry);
                table.add_replaced(None);
            } else {
                table.add_entry(diff);
                table.add_replaced(Some(entry));
            }
        }

        return table;
    }
}
