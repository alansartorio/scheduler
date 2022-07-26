use crate::models::Week;
use crate::models::{
    Building, Day, DaysOfTheWeek, Span, Subject, SubjectCommision, Task, TaskInfo,
};
use enum_map::enum_map;
use itertools::Itertools;
use rusqlite::{named_params, Connection};
use std::cell::RefCell;
use std::error::Error;
use std::rc::{Rc, Weak};

struct CommissionData {
    name: String,
    schedule: Week<TaskInfo>,
}

fn query_tasks_for_day(
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
                        building: Building {
                            name: row.get(1).ok(),
                        },
                        subject: RefCell::new(Weak::new()),
                    },
                ))
            },
        )
        .unwrap()
        .map(Result::unwrap)
        .collect()
}

fn query_subject_commissions(connection: &Connection, subject_code: String) -> Vec<CommissionData> {
    connection
        .prepare("SELECT * FROM commissions WHERE subjectCode = :code")
        .unwrap()
        .query_map(
            named_params! {
                ":code": subject_code,
            },
            |row| {
                Ok(CommissionData {
                    name: row.get(1).unwrap(),
                    schedule: Week::new(enum_map! {
                        day => Day::new(
                            query_tasks_for_day(connection, row.get(2).unwrap(), day)
                    )
                    }),
                })
            },
        )
        .unwrap()
        .into_iter()
        .map(Result::unwrap)
        .collect()
}

pub fn load() -> Result<Vec<Rc<Subject>>, Box<dyn Error>> {
    let connection = rusqlite::Connection::open_with_flags(
        "../data/database.db",
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .unwrap();

    //for building in connection
    //.prepare("SELECT * FROM buildings")
    //.unwrap()
    //.query_map([], |row| Ok(Building { name: row.get(0)? }))?
    //{
    //print!("{:?}", building.unwrap());
    //}
    //dbg!(query_tasks_for_day(&connection, "32422".to_owned(), DaysOfTheWeek::Tuesday));

    let x = connection
        .prepare("SELECT * FROM subjects")
        .unwrap()
        .query_map([], |row| {
            let commissions = query_subject_commissions(&connection, row.get(0).unwrap());

            let code = row.get::<_, String>(0)?.parse().unwrap();
            let name = row.get(1)?;
            let credits = commissions[0]
                .schedule
                .days
                .values()
                .flat_map(|s| &s.tasks)
                .map(|t| (t.span.duration() / 60) as u8)
                .sum();

            let subject = Rc::new_cyclic(|rc| Subject {
                code,
                name,
                credits,
                commissions: commissions
                    .into_iter()
                    .map(|c| SubjectCommision {
                        name: c.name,
                        schedule: c.schedule,
                        subject: rc.clone(),
                    })
                    .collect_vec(),
            });

            Ok(subject)
        })?
        .map(Result::unwrap)
        .collect_vec();
    for sub in &x {
        for com in &sub.commissions {
            for (_day, day_tasks) in &com.schedule.days {
                for task in &day_tasks.tasks {
                    *RefCell::borrow_mut(&task.info.subject) = Rc::downgrade(sub);
                }
            }
        }
    }
    Ok(x)

    //Ok(vec![Subject {
    //code: "01.30".parse().unwrap(),
    //commissions: vec![SubjectCommision {
    //name: "Hola".to_owned(),
    //schedule: Week::new(enum_map! {
    //DaysOfTheWeek::Monday => Day::new(vec![
    //Task::new(Span::new("15:00".parse().unwrap(), "18:00".parse().unwrap()), TaskInfo { building: Building { name: "External".to_owned() } }),
    //]),
    //_ => Day::empty(),
    //}),
    //}],
    //}])
    //let row = cursor.next().unwrap().unwrap();
    //println!("{}", row[0].as_string().unwrap());
}
