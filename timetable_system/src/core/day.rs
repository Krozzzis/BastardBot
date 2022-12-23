#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum WeekDay {
    Monday      = 1,
    Tuesday     = 2,
    Wednesday   = 3,
    Thursday    = 4,
    Friday      = 5,
    Saturday    = 8,
}

#[derive(Clone, Copy, Debug)]
pub enum Day {
    Date(i32),
    WeekDay(WeekDay),
}

impl Day {
    pub fn get_weekday(&self) -> WeekDay {
        match self {
            Day::Date(day) => WeekDay::Monday,
            Day::WeekDay(wd) => *wd,
        }
    }
}
