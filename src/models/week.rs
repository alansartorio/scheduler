use super::collidable::Collidable;
use super::combinable::Combinable;
use crate::models::day::Day;
use enum_map::{enum_map, Enum, EnumMap};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[cfg(feature = "json")]
use serde::Serialize;

#[derive(Debug, Enum, Clone, Copy, EnumIter)]
#[cfg_attr(feature = "json", derive(Serialize))]
#[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
pub enum DaysOfTheWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Week<T> {
    pub days: EnumMap<DaysOfTheWeek, Day<T>>,
}

impl<T> Week<T> {
    pub fn new(days: EnumMap<DaysOfTheWeek, Day<T>>) -> Week<T> {
        Week { days }
    }

    pub fn empty() -> Week<T> {
        Self::new(enum_map! {
            _ => Day::empty()
        })
    }
}
impl<T: Clone> Combinable for Week<T> {
    fn combine(&self, other: &Self) -> Self {
        Week::new(enum_map! {
            day => self.days[day].combine(&other.days[day]),
        })
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
        false
    }
}

