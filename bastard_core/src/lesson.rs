#[derive(Debug, Clone)]
pub struct Lesson {
    pub name: String,
    pub teacher: String,
    pub auditory: String,
    pub replacing: Option<Box<Lesson>>,
}

pub trait LessonFormatter {
    fn format_lesson(&self, lesson: &Lesson) -> String;
}
