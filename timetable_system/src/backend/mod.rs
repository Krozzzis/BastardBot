use crate::{Day, WeekDay, TimeTable, Lesson, Entry, LessonEntry};
use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
struct RawTable {
    lessons: Vec<RawLesson>,
}

#[derive(Deserialize, Debug, Clone)]
struct RawLesson {
    pub subject: String,
    pub auditory: String,
    pub teacher: String,
    pub number: i32,
    pub subgroup: i32,
}

impl RawTable {
    pub fn to_table(&self) -> Result<TimeTable, Box<dyn std::error::Error>> {
        let mut tt = TimeTable::new();
        
        if self.lessons.len() == 0 {
            return Ok(tt);
        }
        let min_num = 1;//self.lessons.iter().min_by(|a, b| a.number.cmp(&b.number)).unwrap().number;
        let max_num = self.lessons.iter().max_by(|a, b| a.number.cmp(&b.number)).unwrap().number;

        for num in min_num..=max_num {
            let mut group: Vec<Option<Lesson>> = Vec::new();
            let mut lessons: Vec<RawLesson> = self.lessons.iter().filter(|x| x.number == num).map(|x| x.clone()).collect();
            lessons.sort_by(|a, b| a.subgroup.cmp(&b.subgroup));
            if lessons.len() == 0 {
                let entry = LessonEntry {
                    position: num,
                    lessons: Vec::new(),
                };
                tt.add_entry(Box::new(entry));
                continue;
            }
            let min_sg = lessons.iter().min_by(|a, b| a.subgroup.cmp(&b.subgroup)).unwrap().subgroup;
            let max_sg = lessons.iter().max_by(|a, b| a.subgroup.cmp(&b.subgroup)).unwrap().subgroup;
            for sg in min_sg..=max_sg {
                match lessons.iter().find(|x| x.subgroup == sg) {
                    Some(a) => {
                        let lesson = Lesson {
                            subject: a.subject.clone(),
                            auditory: a.auditory.clone(),
                            teacher: a.teacher.clone(),
                        };
                        group.push(Some(lesson));
                    },
                    None => {
                        group.push(None);
                    },
                }
            }
            let entry = LessonEntry {
                position: num,
                lessons: group,
            };
            tt.add_entry(Box::new(entry));
        }
        
        return Ok(tt);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TimeTableBackend {
}

impl TimeTableBackend {
    fn get_url(day: Day) -> String {
        let day = day.get_weekday();
        if let WeekDay::Saturday = day {
            return format!("ABOBA");
        }
        format!("https://lyceum.urfu.ru/?type=11&scheduleType=group&weekday={}&group=32", day as i32)
    }

    pub async fn get_timetable(&self, day: Day) -> Result<TimeTable, Box<dyn std::error::Error>> {
        let body = reqwest::get(Self::get_url(day))
            .await?
            .text()
            .await?;
        let raw: RawTable = serde_json::from_str(&body)?;
        raw.to_table()
    }
}
