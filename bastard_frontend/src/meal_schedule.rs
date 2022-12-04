use std::collections::HashMap;
use bastard_core::request_params::Group;

#[derive(Debug, Clone)]
pub struct MealSchedule {
    schedule: HashMap<Group, Vec<MealTime>>,
}

impl MealSchedule {
    pub fn new() -> Self {
        Self {
            schedule: HashMap::new(),
        }
    }
    pub fn by_group(&self, group: Group) -> Option<&Vec<MealTime>> {
        self.schedule.get(&group)
    }

    pub fn add_group(&mut self, group: Group, meals: Vec<MealTime>) {
        self.schedule.insert(group, meals);
    }
}

#[derive(Debug, Clone)]
pub struct MealTime {
    pub place: usize,
    pub kind: String,
}

impl MealTime {
    pub fn new(place: usize, kind: String) -> Self {
        Self {
            place,
            kind
        }
    }
}
