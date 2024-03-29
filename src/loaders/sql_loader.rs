use crate::models::{
    Building, Day, DaysOfTheWeek, Span, Subject, SubjectCommision, Task, TaskInfo,
};
use crate::models::{Code, Week};
use enum_map::enum_map;
use itertools::Itertools;
use rusqlite::{named_params, Connection};
use std::cell::RefCell;
use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, Weak};

fn query_tasks_for_day(
    subject: &Weak<RefCell<Subject>>,
    connection: &Connection,
    commission_id: String,
    day: DaysOfTheWeek,
) -> Vec<Task<TaskInfo>> {
    connection
        .prepare("SELECT * FROM commissionTimes WHERE commissionId = :commission AND day = :day")
        .unwrap()
        .query_map(
            //&[(":commission", &row.get::<_, String>(0)?), (":day", &"MONDAY".to_owned())]
            named_params! {
                ":commission": &commission_id,
                ":day": match day {
                    DaysOfTheWeek::Sunday => "SUNDAY".to_owned(),
                    DaysOfTheWeek::Monday => "MONDAY".to_owned(),
                    DaysOfTheWeek::Tuesday => "TUESDAY".to_owned(),
                    DaysOfTheWeek::Wednesday => "WEDNESDAY".to_owned(),
                    DaysOfTheWeek::Thursday => "THURSDAY".to_owned(),
                    DaysOfTheWeek::Friday => "FRIDAY".to_owned(),
                    DaysOfTheWeek::Saturday => "SATURDAY".to_owned(),
                }
            },
            |row| {
                Ok(Task::new(
                    Span::new(
                        row.get::<_, String>(4).unwrap().parse().unwrap(),
                        row.get::<_, String>(5).unwrap().parse().unwrap(),
                    ),
                    TaskInfo {
                        buildings: row
                            .get(1)
                            .ok()
                            .map(|b| HashSet::from_iter([Building { name: b }]))
                            .unwrap_or_else(HashSet::new),
                        subject: subject.clone(),
                    },
                ))
            },
        )
        .unwrap()
        .map(Result::unwrap)
        .collect()
}

fn query_subject_commissions(
    subject: &Weak<RefCell<Subject>>,
    connection: &Connection,
    subject_code: String,
) -> Vec<SubjectCommision> {
    connection
        .prepare("SELECT * FROM commissions WHERE subjectCode = :code")
        .unwrap()
        .query_map(
            named_params! {
                ":code": subject_code,
            },
            |row| {
                Ok(SubjectCommision {
                    names: vec![row.get(1).unwrap()],
                    schedule: Week::new(enum_map! {
                        day => Day::new(
                            query_tasks_for_day(subject, connection, row.get(2).unwrap(), day)
                        )
                    }),
                    subject: subject.clone(),
                })
            },
        )
        .unwrap()
        .into_iter()
        .map(Result::unwrap)
        .collect()
}

pub fn load() -> Result<Vec<Arc<RefCell<Subject>>>, Box<dyn Error>> {
    let connection = rusqlite::Connection::open_with_flags(
        "../data/database.db",
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .unwrap();

    let x = connection
        .prepare("SELECT * FROM subjects")?
        .query_map([], |row| {
            let code: Code = row.get::<_, String>(0)?.parse().unwrap();
            let name = row.get(1)?;
            Ok(Arc::new_cyclic(|rc| {
                let commissions = query_subject_commissions(rc, &connection, code.to_string());
                let credits = commissions[0]
                    .schedule
                    .days
                    .values()
                    .flat_map(|s| &s.tasks)
                    .map(|t| (t.span.duration() / 60) as u8)
                    .sum();
                RefCell::new(Subject {
                    code,
                    name,
                    credits,
                    commissions,
                })
            }))
        })?
        .map(Result::unwrap)
        .collect_vec();
    Ok(x)
}
