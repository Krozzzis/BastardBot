#[derive(Debug)]
pub struct TimeTable {
    entries: Vec<Box<dyn Entry>>,
}

impl TimeTable {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: Box<dyn Entry>) {
        self.entries.push(entry);
    }
}

impl std::fmt::Display for TimeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in &self.entries {
            write!(f, "{}", entry.to_string())?;
        }
        Ok(())
    }
}
pub trait Entry: std::fmt::Display + std::fmt::Debug {
    fn name(&self) -> &str;
    fn position(&self) -> Option<i32>;
}

#[derive(Debug, Clone)]
pub struct LessonEntry {
    pub position: i32,
    pub lessons: Vec<Option<Lesson>>,
}

impl std::fmt::Display for LessonEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.position() {
            Some(pos) => write!(f, "{}:\n", pos)?,
            None => {},
        };
        if self.lessons.len() == 0 {
            write!(f , "x\n")?;
        }
        for lesson in &self.lessons {
            match lesson {
                Some(a) => write!(f, "{} [{}] - {}\n", a.subject, a.auditory, a.teacher)?,
                None => write!(f, "x\n")?,
            }
        }
        Ok(())
    }
}

impl Entry for LessonEntry {
    fn name(&self) -> &str {
        "Lessons"
    }

    fn position(&self) -> Option<i32> {
        Some(self.position)
    }
}

#[derive(Debug, Clone)]
pub struct Lesson {
    pub subject: String,
    pub auditory: String,
    pub teacher: String,
}
