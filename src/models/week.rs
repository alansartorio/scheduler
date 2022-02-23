use enum_map::{Enum, EnumMap};

use crate::models::day::Day;

#[derive(Debug, Enum)]
pub enum DaysOfTheWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

pub struct Week<T> {
    days: EnumMap<DaysOfTheWeek, Day<T>>,
}

impl<T> Week<T> {
    pub fn new(days: EnumMap<DaysOfTheWeek, Day<T>>) -> Week<T> {
        Week { days }
    }
}
