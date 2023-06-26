use std::ops::Add;

use super::combinable::Combinable;
use super::Span;
use super::{collidable::Collidable, Task};
use crate::models::day::Day;
use enum_map::{enum_map, Enum, EnumMap};
#[cfg(feature = "json")]
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl<T: Add<Output = T> + Clone> Week<T> {
    pub fn simplify(&mut self) {
        for (_, day) in self.days.iter_mut().filter(|(_, day)| day.has_collisions()) {
            let mut new_tasks: Vec<Task<T>> = vec![];
            for task in day.tasks.drain(..) {
                if let Some(last) = new_tasks.last_mut() {
                    if last.span.collides(&task.span) {
                        let start = task.span.start.min(last.span.start);
                        let end = task.span.end.max(last.span.end);

                        let info = task.info + last.info.clone();

                        let new_task = Task::new(Span::new(start, end), info);

                        *last = new_task;
                    } else {
                        new_tasks.push(task)
                    }
                } else {
                    new_tasks.push(task)
                }
            }

            day.tasks.extend(new_tasks);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{Day, DaysOfTheWeek, Span},
        t,
    };
    use enum_map::enum_map;

    #[test]
    fn test_simplify_week() {
        let ta = t!("00:00");
        let tb = t!("01:00");
        let tc = t!("02:00");
        let td = t!("03:00");
        let task_a = Task::new(Span::new(ta, tb), 1);
        let task_b = Task::new(Span::new(ta, tc), 2);
        let task_c = Task::new(Span::new(tc, td), 4);

        let mut week = Week::new(enum_map! {
            DaysOfTheWeek::Monday => Day::new(vec![
                task_a,
                task_b,
                task_c
            ]),
            _ => Day::new(vec![])
        });

        week.simplify();
        week.days
            .iter_mut()
            .for_each(|(_, day)| day.update_has_collissions());

        assert_eq!(
            week,
            Week::new(enum_map! {
                DaysOfTheWeek::Monday => Day::new(vec![
                    Task::new(Span::new(ta, tc), 3),
                    task_c
                ]),
                _ => Day::new(vec![])
            })
        )
    }

    #[test]
    fn test_simplify_week_2() {
        let ta = t!("15:00");
        let tb = t!("18:00");

        let task_a = Task::new(Span::new(ta, tb), 1);
        let task_b = Task::new(Span::new(ta, tb), 2);

        let mut week = Week::new(enum_map! {
            DaysOfTheWeek::Monday => Day::new(vec![
                task_a,
                task_b
            ]),
            _ => Day::new(vec![])
        });

        week.simplify();
        week.days
            .iter_mut()
            .for_each(|(_, day)| day.update_has_collissions());

        assert_eq!(
            week,
            Week::new(enum_map! {
                DaysOfTheWeek::Monday => Day::new(vec![
                    Task::new(Span::new(ta, tb), 3),
                ]),
                _ => Day::new(vec![])
            })
        )
    }
}
