#[derive(Debug, Clone)]
pub struct Lesson {
    pub name: String,
    pub teacher: String,
    pub auditory: String,
}

pub trait LessonFormatter {
    fn format_lesson(&self, lesson: Option<&Lesson>) -> String;
}
