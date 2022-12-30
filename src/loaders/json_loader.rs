use crate::models::*;
use enum_map::enum_map;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

fn map(
    parsed: json_parser::SubjectCommissions,
) -> Result<Vec<Arc<RefCell<Subject>>>, Box<dyn Error>> {
    Ok(parsed
        .0
        .iter()
        .group_by(|s| (s.subject_code, &s.subject_name))
        .into_iter()
        .map(|((code, name), commissions)| {
            Arc::new_cyclic(|sub| {
                let list = commissions.collect_vec();
                let commissions = list
                    .iter()
                    .map(|c| SubjectCommision {
                        names: vec![c.commission_name.clone()],
                        subject: sub.clone(),
                        schedule: Week::new(enum_map! {
                            day => {
                                let day = match day {
                                    DaysOfTheWeek::Monday => json_parser::Day::Monday,
                                    DaysOfTheWeek::Tuesday => json_parser::Day::Tuesday,
                                    DaysOfTheWeek::Wednesday => json_parser::Day::Wednesday,
                                    DaysOfTheWeek::Thursday => json_parser::Day::Thursday,
                                    DaysOfTheWeek::Friday => json_parser::Day::Friday,
                                    DaysOfTheWeek::Saturday => json_parser::Day::Saturday,
                                    DaysOfTheWeek::Sunday => json_parser::Day::Sunday,
                                };
                                Day::new(
                                    c.course_commission_times
                                    .0 .iter() .filter(|t| t.day == day)
                                    .map(|t| Task::new(
                                            Span::new(
                                                Time::new(t.span.start.hours, t.span.start.minutes),
                                                Time::new(t.span.end.hours, t.span.end.minutes)
                                            ),
                                            TaskInfo {
                                                subject: sub.clone(),
                                                building: HashSet::from_iter([Building {
                                                    name: t.building.clone()
                                                }])
                                            }
                                    )
                                    ).collect_vec())
                            }
                        }),
                    })
                    .collect_vec();
                let credits = commissions[0]
                    .schedule
                    .days
                    .values()
                    .flat_map(|s| &s.tasks)
                    .map(|t| (t.span.duration() / 60) as u8)
                    .sum();
                RefCell::new(Subject {
                    code: Code {
                        high: code.high,
                        low: code.low,
                    },
                    name: name.clone(),
                    credits,
                    commissions,
                })
            })
        })
        .collect_vec())
}

pub fn load(path: &Path) -> Result<Vec<Arc<RefCell<Subject>>>, Box<dyn Error>> {
    let reader = File::open(path)?;
    load_from_reader(reader)
}

pub fn load_from_reader<R: Read>(reader: R) -> Result<Vec<Arc<RefCell<Subject>>>, Box<dyn Error>> {
    let parsed: json_parser::SubjectCommissions = serde_json::from_reader(reader)?;
    map(parsed)
}

pub fn load_from_string(string: &str) -> Result<Vec<Arc<RefCell<Subject>>>, Box<dyn Error>> {
    let parsed = serde_json::from_str(string)?;
    map(parsed)
}
