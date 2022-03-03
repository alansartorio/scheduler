use enum_map::{Enum, EnumMap, enum_map};

use crate::models::day::Day;

use super::collidable::Collidable;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use super::combinable::Combinable;

#[derive(Debug, Enum, Clone, Copy, EnumIter)]
pub enum DaysOfTheWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug)]
pub struct Week<T> {
    days: EnumMap<DaysOfTheWeek, Day<T>>,
}

impl<T> Week<T> {
    pub fn new(days: EnumMap<DaysOfTheWeek, Day<T>>) -> Week<T> {
        Week { days }
    }

}
impl<T: Clone> Combinable for Week<T> {
    fn combine(&self, other: &Self) -> Self {
        Week::new(
            enum_map! {
                day => self.days[day].combine(&other.days[day]),
            }
        )
    }
}

impl<T> Collidable for Week<T> {

    fn collides(&self, other: &Self) -> bool {
        for day in DaysOfTheWeek::iter() {
            let day1 = &self.days[day];
            let day2 = &other.days[day];
            if day1.collides(day2) {
                return true;
            }
        }
        return false;
    }
}
