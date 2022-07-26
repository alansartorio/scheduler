use crate::models::*;
use enum_map::enum_map;
use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

pub fn load(path: &Path) -> Result<Vec<Rc<Subject>>, Box<dyn Error>> {
    let reader = File::open(path)?;
    let parsed: json_parser::SubjectCommissions = serde_json::from_reader(&reader)?;
    Ok(parsed
        .0
        .iter()
        .group_by(|s| s.subject_code)
        .into_iter()
        .map(|(code, commissions)| {
            Rc::new_cyclic(|sub| {
                let list = commissions.collect_vec();
                let commissions = list
                    .iter()
                    .map(|c| SubjectCommision {
                        name: c.commission_name.clone(),
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
                                    .0
                                    .iter()
                                    .filter(|t| t.day == day)
                                    .map(|t| Task::new(
                                            Span::new(
                                                Time::new(t.span.start.hours, t.span.start.minutes),
                                                Time::new(t.span.end.hours, t.span.end.minutes)
                                            ),
                                            TaskInfo {
                                                subject: sub.clone(),
                                                building: Building {
                                                    name: Some(t.building.clone())
                                                }
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
                Subject {
                    code: Code {
                        high: code.high,
                        low: code.low,
                    },
                    name: list[0].subject_name.clone(),
                    credits,
                    commissions,
                }
            })
        })
        .collect_vec())
}