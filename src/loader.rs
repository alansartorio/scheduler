use itertools::Itertools;
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    hash::Hash,
    rc::Weak,
};
pub use std::{error::Error, rc::Rc, str::FromStr, string::ParseError};

use enum_map::enum_map;
use rusqlite::{named_params, Connection};

use crate::models::{
    collidable::Collidable,
    day::Day,
    span::Span,
    task::Task,
    week::{DaysOfTheWeek, Week},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Building {
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub subject: RefCell<Weak<Subject>>,
    pub building: Building,
}

#[derive(Debug, Clone)]
pub struct SubjectCommision {
    pub name: String,
    pub subject: RefCell<Weak<Subject>>,
    pub schedule: Week<TaskInfo>,
}
impl Display for SubjectCommision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({})",
            RefCell::borrow(&self.subject).upgrade().unwrap(),
            self.name
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
        self.name.hash(state);
        self.subject.borrow().upgrade().unwrap().code.hash(state);
    }
}
impl PartialEq for SubjectCommision {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
            && self.subject.borrow().upgrade().unwrap().eq(&other
                .subject
                .borrow()
                .upgrade()
                .unwrap())
    }
}
impl Eq for SubjectCommision {}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Code {
    high: u8,
    low: u8,
}
impl FromStr for Code {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (high, low) = s
            .split_once('.')
            .ok_or("Subject Code must contain a dot.")?;
        let high: u8 = high.parse()?;
        let low: u8 = low.parse()?;
        Ok(Code { high, low })
    }
}
impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}.{:02}", self.high, self.low)
    }
}
impl Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Hash, PartialEq)]
pub struct Subject {
    pub code: Code,
    pub name: String,
    pub commissions: Vec<SubjectCommision>,
    pub credits: u8,
}

impl Eq for Subject {}
impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} ({})", self.code, self.name, self.credits)
    }
}
impl Subject {
    pub fn find_commission_by_id<'a>(&'a self, id: &str) -> Option<&'a SubjectCommision> {
        self.commissions.iter().find(|com| com.name == id)
    }
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

fn query_subject_commissions(
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
                    name: row.get(1).unwrap(),
                    subject: RefCell::new(Weak::new()),
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
            Ok(Rc::new(Subject {
                code: row.get::<_, String>(0)?.parse().unwrap(),
                name: row.get(1)?,
                credits: commissions[0]
                    .schedule
                    .days
                    .values()
                    .flat_map(|s| &s.tasks)
                    .map(|t| (t.span.duration() / 60) as u8)
                    .sum(),
                commissions,
            }))
        })?
        .map(Result::unwrap)
        .collect_vec();
    for sub in &x {
        for com in &sub.commissions {
            *RefCell::borrow_mut(&com.subject) = Rc::downgrade(sub);
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
