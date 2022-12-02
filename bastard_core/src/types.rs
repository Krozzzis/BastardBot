#[derive(Debug, Clone)]
pub struct Lesson {
    pub name: String,
    pub teacher: String,
    pub auditory: String,
}

#[derive(Debug, Clone)]
pub enum Entry {
    NoLessons,
    OneLesson(Lesson),
    TwoLessons(Option<Lesson>, Option<Lesson>),
}

#[derive(Debug, Clone)]
pub struct Section {
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub sections: Vec<Section>, 
}

impl Table {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }
    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Group {
    _11A = 6,
    _11B = 7,
    _11V = 8,
    _11G = 24,
    _11D = 25,
    _11E = 26,
    _11Z = 27,
    _11N = 32,
}


#[derive(Debug, Clone)]
pub enum DayOfWeek {
    Monday      = 1,
    Tuesday     = 2,
    Wednesday   = 3,
    Thursday    = 4,
    Friday      = 5,
    Saturday    = 6,
}
