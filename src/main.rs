use bastard_backend::WebParser;
use bastard_frontend::SimpleFormatter;
use bastard_frontend::meal_schedule::{MealSchedule, MealTime};
use bastard_core::request_params::{DayOfWeek, Group};
use bastard_core::schedule_table::{ScheduleTableGenerator, ScheduleTableFormatter};

fn main() {
    let group = Group::_11N;
    let weekday = DayOfWeek::Tuesday;
    let parser = WebParser::new("https://lyceum.urfu.ru/".to_string());
    let table = match parser.get_table(group, weekday) {
        Ok(a) => a,
        Err(e) => {
            println!("⚠ Error: {}", e);
            return
        },
    };
    let mut meal_schedule = MealSchedule::new();
    meal_schedule.add_group(Group::_11N, vec![
        MealTime::new(3, String::from("Завтрак")),
        MealTime::new(6, String::from("Обед")),
        MealTime::new(7, String::from("Полдник")),
    ]);
    meal_schedule.add_group(Group::_11A, vec![
        MealTime::new(2, String::from("Завтрак")),
        MealTime::new(5, String::from("Обед")),
        MealTime::new(7, String::from("Полдник")),
    ]);
    let formatter = SimpleFormatter::new(
        Some(vec![
            (String::from("9:00"), String::from("9:40")),
            (String::from("9:50"), String::from("10:30")),
            (String::from("10:45"), String::from("11:25")),
            (String::from("11:40"), String::from("12:20")),
            (String::from("12:35"), String::from("13:15")),
            (String::from("13:35"), String::from("14:15")),
            (String::from("14:35"), String::from("15:15")),
        ]), Some(meal_schedule), group);
    let output = formatter.format(&table);
    println!("{}", output);
}
