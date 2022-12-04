#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
