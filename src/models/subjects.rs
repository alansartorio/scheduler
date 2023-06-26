use itertools::Itertools;
pub use json_parser::Code;

use crate::models::{Collidable, Week};
use std::{
    cell::RefCell,
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, BitOr},
    sync::Weak,
};
pub use std::{error::Error, rc::Rc, str::FromStr, string::ParseError};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Building {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub subject: Weak<RefCell<Subject>>,
    pub buildings: HashSet<Building>,
}

impl PartialEq for TaskInfo {
    fn eq(&self, other: &Self) -> bool {
        self.buildings == other.buildings
    }
}

impl Eq for TaskInfo {}

impl Hash for TaskInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buildings.iter().for_each(|b| b.hash(state))
    }
}

impl Add for TaskInfo {
    type Output = TaskInfo;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.subject.ptr_eq(&rhs.subject));
        let mut new_building = HashSet::new();
        new_building.extend(self.buildings.into_iter());
        new_building.extend(rhs.buildings.into_iter());
        TaskInfo {
            subject: self.subject,
            buildings: new_building,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubjectCommision {
    pub names: Vec<String>,
    pub subject: Weak<RefCell<Subject>>,
    pub schedule: Week<TaskInfo>,
}
impl Display for SubjectCommision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({:?})",
            self.subject.upgrade().unwrap().borrow(),
            self.names
        )
    }
}
impl Collidable for SubjectCommision {
    fn collides(&self, other: &Self) -> bool {
        self.schedule.collides(&other.schedule)
    }
}
impl Hash for SubjectCommision {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.names.hash(state);
        self.subject.upgrade().unwrap().borrow().code.hash(state);
    }
}
impl PartialEq for SubjectCommision {
    fn eq(&self, other: &Self) -> bool {
        self.names.eq(&other.names) && self.schedule == other.schedule
    }
}
impl Eq for SubjectCommision {}

impl BitOr<&SubjectCommision> for &SubjectCommision {
    type Output = SubjectCommision;
    fn bitor(self, rhs: &SubjectCommision) -> Self::Output {
        assert!(self.subject.ptr_eq(&rhs.subject));
        assert_eq!(self.schedule, rhs.schedule);
        let mut names = self.names.clone();
        names.append(&mut rhs.names.clone());
        SubjectCommision {
            names,
            subject: self.subject.clone(),
            schedule: self.schedule.clone(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Subject {
    pub code: Code,
    pub name: String,
    pub commissions: Vec<SubjectCommision>,
    pub credits: u8,
}

impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} ({})", self.code, self.name, self.credits)
    }
}
impl Subject {
    pub fn find_commission_by_id<'a>(&'a self, id: &str) -> Option<&'a SubjectCommision> {
        self.commissions
            .iter()
            .find(|com| com.names.iter().any(|name| name == id))
    }

    pub fn optimize(&mut self) {
        for com in self.commissions.iter_mut() {
            com.schedule.simplify();

            com.schedule
                .days
                .iter_mut()
                .for_each(|(_, day)| day.update_has_collissions());
        }

        let new_comms = {
            let eq_groups = self
                .commissions
                .drain(..)
                .into_group_map_by(|com| com.schedule.clone());
            eq_groups
                .into_values()
                .map(|group| group.into_iter().reduce(|a, b| (&a | &b)).unwrap())
                .collect_vec()
        };

        self.commissions.extend(new_comms);
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, sync::Arc, vec};

    use enum_map::enum_map;

    use crate::models::*;

    use super::*;

    #[test]
    fn code_to_string() {
        assert_eq!(Code { high: 3, low: 6 }.to_string(), "03.06");
        assert_eq!(Code { high: 10, low: 40 }.to_string(), "10.40");
    }

    #[test]
    fn optimize_subject() {
        let ta = "00:00".parse().unwrap();
        let tb = "01:00".parse().unwrap();
        let tc = "02:00".parse().unwrap();
        let building = [];
        let task_a_1 = |subject: &Weak<RefCell<Subject>>| {
            Task::new(
                Span::new(ta, tb),
                TaskInfo {
                    subject: subject.clone(),
                    buildings: HashSet::from_iter(building.clone()),
                },
            )
        };
        let task_a_2 = |subject: &Weak<RefCell<Subject>>| {
            Task::new(
                Span::new(ta, tb),
                TaskInfo {
                    subject: subject.clone(),
                    buildings: HashSet::from_iter(building.clone()),
                },
            )
        };
        let task_b = |subject: &Weak<RefCell<Subject>>| {
            Task::new(
                Span::new(tb, tc),
                TaskInfo {
                    subject: subject.clone(),
                    buildings: HashSet::from_iter(building.clone()),
                },
            )
        };

        let week_1 = |subject: &Weak<RefCell<Subject>>| {
            Week::new(enum_map! {
                DaysOfTheWeek::Monday => Day::new(vec![
                    task_a_1(subject),
                ]),
                _ => Day::new(vec![])
            })
        };

        let week_2 = |subject: &Weak<RefCell<Subject>>| {
            Week::new(enum_map! {
                DaysOfTheWeek::Monday => Day::new(vec![
                    task_a_2(subject),
                ]),
                _ => Day::new(vec![])
            })
        };

        let week_3 = |subject: &Weak<RefCell<Subject>>| {
            Week::new(enum_map! {
                DaysOfTheWeek::Monday => Day::new(vec![
                    task_b(subject),
                ]),
                _ => Day::new(vec![])
            })
        };

        let subject = Arc::new_cyclic(|subject| {
            RefCell::new(Subject {
                commissions: vec![
                    SubjectCommision {
                        subject: subject.clone(),
                        names: vec!["Com A".to_owned()],
                        schedule: week_1(subject),
                    },
                    SubjectCommision {
                        subject: subject.clone(),
                        names: vec!["Com B".to_owned()],
                        schedule: week_3(subject),
                    },
                    SubjectCommision {
                        subject: subject.clone(),
                        names: vec!["Com C".to_owned()],
                        schedule: week_2(subject),
                    },
                ],

                code: "00.00".parse().unwrap(),
                name: "Nombre".to_owned(),
                credits: 3,
            })
        });

        subject.borrow_mut().optimize();

        let subject_2 = Arc::new_cyclic(|subject| {
            RefCell::new(Subject {
                code: "00.00".parse().unwrap(),
                name: "Nombre".to_owned(),
                commissions: vec![
                    SubjectCommision {
                        subject: subject.clone(),
                        names: vec!["Com B".to_owned()],
                        schedule: week_3(subject),
                    },
                    SubjectCommision {
                        subject: subject.clone(),
                        names: vec!["Com A".to_owned(), "Com C".to_owned()],
                        schedule: week_1(subject),
                    },
                ],
                credits: 3,
            })
        });

        assert_eq!(
            subject
                .borrow()
                .commissions
                .iter()
                .map(|c| c.names.iter().sorted().collect_vec())
                .collect::<HashSet<_>>(),
            subject_2
                .borrow()
                .commissions
                .iter()
                .map(|c| c.names.iter().sorted().collect_vec())
                .collect::<HashSet<_>>(),
        );
    }
}
